import re
from util import util
import unicodedata
import bs4.element
from classes.author import Author
from classes.tag import Tag
from typing import List


class Story:
    def __init__(self, id_: int, title: str, image: bool, author: Author, rating: str, tag_list: List[Tag],
                 words: int, views: int, thumbs_up: int, thumbs_down: int, approved_rank: int = None, updated_rank: int = None, heat_rank: int = None,
                 front_featured_rank: int = None, front_approved_rank: int = None, front_updated_rank: int = None,
                 front_heat_rank: int = None):
        # Create a story from pre-parsed data
        self.id = id_
        self.title = title
        self.image = image
        self.author = author
        self.rating = rating
        self.tag_list = tag_list
        self.words = words
        self.views = views
        self.thumbs_up = thumbs_up
        self.thumbs_down = thumbs_down
        self.approved_rank = approved_rank
        self.updated_rank = updated_rank
        self.heat_rank = heat_rank
        self.front_featured_rank = front_featured_rank
        self.front_approved_rank = front_approved_rank
        self.front_updated_rank = front_updated_rank
        self.front_heat_rank = front_heat_rank

    @staticmethod
    def create_from_story_card(story_card: bs4.element.Tag, approved_rank: int = None,
                               updated_rank: int = None, heat_rank: int = None, front_featured_rank: int = None,
                               front_approved_rank: int = None, front_updated_rank: int = None,
                               front_heat_rank: int = None):
        # Create a story from unparsed bs4 story card
        
        # TODO: This function cannot be used to parse stories from the feature box or the front heat list. 
        #  Should remove those relevant parameters (front_approved_rank, front_heat_rank)
        #  (and specify arguments in the return statement, to prevent bugs...)

        my_story = {}

        # Grab story rating
        my_story['rating'] = story_card.find(class_=re.compile('content_rating.*')).contents[0]

        # Grab story ID
        story_href = story_card.find(class_='story_link').attrs['href']
        my_story['id'] = int(re.search(r'/story/([0-9]+)/.*', story_href).group(1))

        # Grab story title
        my_story['title'] = story_card.find(class_='story_link').attrs['title']

        # Grab story image (true or false: whether story has an image)
        my_story['image'] = bool(story_card.find(class_='story-image'))

        # Grab tags
        my_story['tags'] = []
        tags = story_card.find_all(class_=re.compile('tag-.*'))
        for tag in tags:
            my_story['tags'].append(Tag(tag.attrs['data-tag-id'], tag.attrs['class'][0],
                                        tag.attrs['data-tag'], tag.contents))

        # Grab author
        author = story_card.find(class_='story-card__author')
        my_story['author'] = Author(author.attrs['href'], author.contents)

        info = story_card.find(class_='story-card__info').contents

        # Grab words
        my_story['words'] = util.parse_number(re.search(r'([0-9.,]+([km])?) words', info[3:].__str__()).group(1))

        # Grab views
        my_story['views'] = util.parse_number(re.search(r'([0-9]+([km])?) views', info[3:].__str__()).group(1))

        # unicodedata.normalize('NFKD', info[12]).strip()

        # Grab thumbs
        if story_card.find(class_='fa fa-thumbs-up'):
            thumbs_up = story_card.find(class_='fa fa-thumbs-up').next_element
            my_story['thumbs_up'] = util.parse_number(unicodedata.normalize('NFKD', thumbs_up).strip())
        else:
            my_story['thumbs_up'] = 0

        if story_card.find(class_='fa fa-thumbs-down'):
            thumbs_down = story_card.find(class_='fa fa-thumbs-down').next_element
            my_story['thumbs_down'] = util.parse_number(unicodedata.normalize('NFKD', thumbs_down).strip())
        else:
            my_story['thumbs_down'] = 0

        # my_story['heat_rank'] = index

        # (self, id_: int, title: str, author: Author, rating: str, tag_list: List[Tag],
        #                  words: int, views: int, thumbs_up: int, thumbs_down: int)

        # Return a story
        return Story(my_story['id'], my_story['title'], my_story['image'], my_story['author'], my_story['rating'], my_story['tags'],
                     my_story['words'], my_story['views'], my_story['thumbs_up'], my_story['thumbs_down'],
                     approved_rank, updated_rank, heat_rank, front_featured_rank, front_approved_rank,
                     front_updated_rank, front_heat_rank)
    
    # @staticmethod
    # def create_from_featured_story(featured_story: bs4.element.Tag, front_featured_rank: int):
    #     # Create a story from unparsed bs4 story card
    #
    #     my_story = {}
    #
    #     # TODO: Put featured story parsing here!!!
    #
    #     title = featured_story.select_one('div.title')
    #     desc = featured_story.select_one('div.description')
    #     info = featured_story.select_one('div.info')
    #
    #     # Grab story rating
    #     my_story['rating'] = title.find(class_=re.compile('content_rating.*')).contents[0]
    #
    #     # Grab story ID
    #     story_href = title.select_one('a').attrs['href']
    #     my_story['id'] = int(re.search(r'/story/([0-9]+)/.*', story_href).group(1))
    #
    #     # Grab story title
    #     my_story['title'] = title.select_one('a').contents[0]
    #
    #     # Grab tags
    #     my_story['tags'] = []
    #     tags = desc.find_all(class_=re.compile('tag-.*'))
    #     for tag in tags:
    #         my_story['tags'].append(Tag(tag.attrs['data-tag-id'], tag.attrs['class'][0],
    #                                     tag.attrs['data-tag'], tag.contents))
    #
    #     # Grab author
    #     author = featured_story.find(class_='story-card__author')
    #     my_story['author'] = Author(author.attrs['href'], author.contents)
    #
    #     info = featured_story.find(class_='story-card__info').contents
    #
    #     # Grab words
    #     my_story['words'] = util.parse_number(re.search(r'([0-9.,]+([km])?) words', info[3:].__str__()).group(1))
    #
    #     # Grab views
    #     my_story['views'] = util.parse_number(re.search(r'([0-9]+([km])?) views', info[3:].__str__()).group(1))
    #
    #     # unicodedata.normalize('NFKD', info[12]).strip()
    #
    #     # Grab thumbs
    #     if featured_story.find(class_='fa fa-thumbs-up'):
    #         thumbs_up = featured_story.find(class_='fa fa-thumbs-up').next_element
    #         my_story['thumbs_up'] = util.parse_number(unicodedata.normalize('NFKD', thumbs_up).strip())
    #     else:
    #         my_story['thumbs_up'] = 0
    #
    #     if featured_story.find(class_='fa fa-thumbs-down'):
    #         thumbs_down = featured_story.find(class_='fa fa-thumbs-down').next_element
    #         my_story['thumbs_down'] = util.parse_number(unicodedata.normalize('NFKD', thumbs_down).strip())
    #     else:
    #         my_story['thumbs_down'] = 0
    #
    #     # Return a story
    #     return Story(my_story['id'], my_story['title'], my_story['author'], my_story['rating'], my_story['tags'],
    #                  my_story['words'], my_story['views'], my_story['thumbs_up'], my_story['thumbs_down'],
    #                  front_featured_rank=front_featured_rank)
    #
    # @staticmethod
    # def create_from_popular_story(popular_story: bs4.element.Tag, front_heat_rank: int):
    #     # Create a story from unparsed bs4 story card
    #
    #     my_story = {}
    #
    #     # TODO: Put popular story parsing here!!!
    #
    #     # Return a story
    #     return Story(my_story['id'], my_story['title'], my_story['author'], my_story['rating'], my_story['tags'],
    #                  my_story['words'], my_story['views'], my_story['thumbs_up'], my_story['thumbs_down'],
    #                  front_heat_rank=front_heat_rank)
