import json
from fimficdata.common import util, sql_patterns
import time
import re
import sqlite3
import pathlib

from datetime import datetime
from random import randint

import requests
from bs4 import BeautifulSoup

from fimficdata.common.story import Story
from fimficdata.common.storylist import StoryList
from fimficdata.schedulers.schedule import Schedule

# Debug setting: avoids scheduler, just runs once & immediately
RUN_ONCE = False
# Debug setting: avoids running the scraper
RUN_TEST = False

# Number of minutes between cycles
INTERVAL = 15

# Number of seconds to delay before beginning a scrape of each page within a cycle
DELAY_BETWEEN_PAGES = 5


# Reference for extracting header/cookie info for login: https://stackoverflow.com/a/61140905
# Reference for sqlite3 commit/rollback info: https://stackoverflow.com/a/23634805

# login_cookies = None
# login_headers = None

project_root = str(pathlib.Path(__file__).parent.parent.parent) + '/'

with open(project_root + 'login_metadata.json') as raw:
    parsed = json.load(raw)
    cookies = parsed['cookies']
    headers = parsed['headers']


def grab_stories(login_cookies, login_headers):
    # Extract & format available stories
    urls = {'frontpage': 'https://www.fimfiction.net/',
            'approved': 'https://www.fimfiction.net/stories?order=latest&view_mode=2',
            'updated': 'https://www.fimfiction.net/stories?order=updated&view_mode=2',
            'heat': 'https://www.fimfiction.net/stories?order=heat&view_mode=2'}

    # recently posted = 'https://www.fimfiction.net/stories?order=latest&view_mode=2'
    # recently updated = 'https://www.fimfiction.net/stories?order=updated&view_mode=2'
    # hot = 'https://www.fimfiction.net/stories?order=heat&view_mode=2'

    # fimfiction = requests.get('https://www.fimfiction.net/stories?order=latest&view_mode=2', cookies=login_cookies, headers=login_headers)

    # TODO: Setup error handling & request retry here

    fimfiction = {'approved': requests.get(urls['approved'], cookies=login_cookies, headers=login_headers),
                  'updated': requests.get(urls['updated'], cookies=login_cookies, headers=login_headers),
                  'heat': requests.get(urls['heat'], cookies=login_cookies, headers=login_headers),
                  'frontpage': requests.get(urls['frontpage'], cookies=login_cookies, headers=login_headers)}

    story_list = StoryList()

    all_stories = {}

    for page in fimfiction:

        time.sleep(DELAY_BETWEEN_PAGES)

        soup = BeautifulSoup(fimfiction[page].content, 'html.parser')

        # Grab the usercount from the bottom of the page
        usercount_raw = soup.find('div', class_='footer').find('div', class_='block').find('p').contents[8].contents[0]
        story_list.add_usercount(util.parse_number(usercount_raw))

        totalhits_raw = soup.find('div', class_='footer').find('div', class_='block').find('p').contents[13].contents[0]
        story_list.set_totalhits(util.parse_number(totalhits_raw))

        if page in ['updated', 'approved', 'heat']:
            story_cards = soup.select_one('.story-card-list').select('.story-card-container')

            for index, story_card in enumerate(story_cards, start=1):
                # Set rank based on index as it appears in the list
                if page == 'updated':
                    my_story = Story.create_from_story_card(story_card, updated_rank=index)
                elif page == 'approved':
                    my_story = Story.create_from_story_card(story_card, approved_rank=index)
                else:  # page == 'heat'
                    my_story = Story.create_from_story_card(story_card, heat_rank=index)

                story_list.add_story(my_story)

        if page == 'frontpage':
            # Some sort of implementation that can grab all the stories from the front page, including feature box
            featured = soup.select('.featured_story')
            approved = soup.select_one('#new_stories').select('.story-card-container')
            updated = soup.select_one('#latest_stories').select('.story-card-container')
            heat = soup.select_one('#popular_stories').select('.story-card-container')

            for index, story_card in enumerate(featured, start=1):
                story_href = story_card.select_one('div.title').select_one('a').attrs['href']
                story_id = int(re.search(r'/story/([0-9]+)/.*', story_href).group(1))
                try:
                    story_list.set_rank(story_id, front_featured_rank=index)
                except KeyError as ke:
                    print(ke)

            for index, story_card in enumerate(approved, start=1):
                story_list.add_story(Story.create_from_story_card(story_card, front_approved_rank=index))

            for index, story_card in enumerate(updated, start=1):
                story_list.add_story(Story.create_from_story_card(story_card, front_updated_rank=index))

            for index, story_card in enumerate(heat, start=1):
                story_href = story_card.find(class_='story_link').attrs['href']
                story_id = int(re.search(r'/story/([0-9]+)/.*', story_href).group(1))
                try:
                    story_list.set_rank(story_id, front_heat_rank=index)
                except KeyError as ke:
                    print(ke)

    return story_list


def register_story(conn_: sqlite3.Connection, story: Story, timestamp_: int):
    # Register a story into the database
    # TODO: Setup rollback & error handling

    cur = conn_.cursor()

    cur.execute("begin")

    # authors
    cur.execute(sql_patterns.insert_authors(story.author.id, story.author.name))

    # tags
    for tag in story.tag_list:
        cur.execute(sql_patterns.insert_tags(tag.id, tag.type, tag.data, tag.name))

    # stories
    cur.execute(sql_patterns.insert_stories(story.id, story.title, story.image, story.rating, story.author.id))

    # tag_links
    for tag in story.tag_list:
        cur.execute(sql_patterns.insert_tag_links(story.id, tag.id))

    # TODO: Update this to include totalhits
    # updates
    cur.execute(sql_patterns.insert_updates(story.id, timestamp_, story.words, story.views, story.thumbs_up,
                                            story.thumbs_down, story.approved_rank, story.updated_rank, story.heat_rank,
                                            story.front_featured_rank, story.front_approved_rank,
                                            story.front_updated_rank, story.front_heat_rank))

    cur.execute("commit")


def update():
    # This function runs periodically to extract an update from FimFiction
    start_time = int(time.time())

    stories = grab_stories(cookies, headers)

    # Get the timestamp for the last 10 minute mark
    # timestamp = util.get_tenth_minute()
    timestamp = util.last_nth_minute(INTERVAL)

    db = project_root + 'fimfic-data.db'
    conn = None
    try:
        conn = sqlite3.connect(db)
        # Disable autocommit
        conn.isolation_level = None
    except sqlite3.Error as e:
        print(e)
        return

    try:
        # Begin transaction
        conn.cursor().execute("begin")

        # Establish & insert usercounts
        conn.cursor().execute(sql_patterns.create_usercounts())
        conn.cursor().execute(sql_patterns.insert_usercounts(timestamp, stories.get_usercount()))

        # Establish tables for story-related data
        conn.cursor().execute(sql_patterns.create_authors())
        conn.cursor().execute(sql_patterns.create_tags())
        conn.cursor().execute(sql_patterns.create_stories())
        conn.cursor().execute(sql_patterns.create_tag_links())
        conn.cursor().execute(sql_patterns.create_updates())

        # Commit transaction
        conn.cursor().execute("commit")
    except sqlite3.Error as e:
        # Roll back transaction in case of failure
        conn.cursor().execute("rollback")
        print(e)
        return

    for story in stories.dict.values():
        register_story(conn, story, timestamp)

    end_time = int(time.time())

    print('Processing {0} stories at {1} took {2}s'
          .format(len(stories.dict.keys()),
                  datetime.utcfromtimestamp(timestamp).strftime('%Y-%m-%d %H:%M:%S'),
                  end_time - start_time))


if RUN_ONCE:
    if RUN_TEST:
        start_time = int(time.time())
        time.sleep(randint(20, 30))
        end_time = int(time.time())

        print('Processing X stories at {0} took {1}s'
              .format(datetime.utcfromtimestamp(start_time).strftime('%Y-%m-%d %H:%M:%S'),
                      end_time - start_time))
    else:  # if not RUN_TEST
        update()

else:  # if not RUN_ONCE
    sched = Schedule(interval=INTERVAL)

    if RUN_TEST:
        print('The scheduled TEST is now running!')

        sched.wait()

        start_time = int(time.time())
        time.sleep(randint(20, 30))
        end_time = int(time.time())

        print('Processing X stories at {0} took {1}s'
              .format(datetime.utcfromtimestamp(start_time).strftime('%Y-%m-%d %H:%M:%S'),
                      end_time - start_time))

    else:  # if not RUN_TEST
        print('The scheduled task is now running!')

        while True:
            sched.wait()
            update()
