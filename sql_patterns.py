import util.util as util


# TODO: Update this to include totalhits
def create_usercounts():
    return """
        CREATE TABLE IF NOT EXISTS usercounts (
            timestamp integer NOT NULL,
            usercount integer NOT NULL,
            
            CONSTRAINT usercounts_pk PRIMARY KEY (timestamp, usercount)
        )"""


# TODO: Update this to include totalhits
def insert_usercounts(timestamp: int, usercount: int):
    return """
        INSERT OR IGNORE INTO usercounts (timestamp, usercount) 
        VALUES ({0}, {1})
        """.format(timestamp, usercount)


def create_stories():
    return """
        CREATE TABLE IF NOT EXISTS stories (
            id        integer PRIMARY KEY,
            title     text    NOT NULL,
            image     integer NOT NULL,
            rating    text    NOT NULL,
            author_id integer NOT NULL,
            
            CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
                REFERENCES authors (id)
        )"""


def insert_stories(id_: int, title: str, image: bool, rating: str, author_id: int):

    title = util.format_quote_strings(title)
    image = 'TRUE' if image else 'FALSE'

    return """
        INSERT OR IGNORE INTO stories (id, title, image, rating, author_id)
        VALUES ({0}, "{1}", {2}, "{3}", {4})
        """.format(id_, title, image, rating, author_id)


def create_authors():
    return """
        CREATE TABLE IF NOT EXISTS authors (
            id   integer PRIMARY KEY,
            name text    NOT NULL
        )"""


def insert_authors(id_: int, name: str):

    name = util.format_quote_strings(name)

    return """
        INSERT OR IGNORE INTO authors (id, name)
        VALUES ({0}, "{1}")
        """.format(id_, name)


def create_tags():
    return """
        CREATE TABLE IF NOT EXISTS tags (
            id   integer PRIMARY KEY,
            type text    NOT NULL,
            data text    NOT NULL,
            name text    NOT NULL
        )"""


def insert_tags(id_: int, type: str, data: str, name: str):
    return """
        INSERT OR IGNORE INTO tags (id, type, data, name)
        VALUES ({0}, "{1}", "{2}", "{3}")
        """.format(id_, type, data, name)


def create_tag_links():
    return """
        CREATE TABLE IF NOT EXISTS tag_links (
            story_id integer,
            tag_id   integer,
            
            CONSTRAINT tag_links_story_id_fk FOREIGN KEY (story_id)
                REFERENCES stories (id),
            CONSTRAINT tag_links_tag_id_fk FOREIGN KEY (tag_id)
                REFERENCES tags (id),
            CONSTRAINT tag_links_pk PRIMARY KEY (story_id, tag_id)
        )"""


def insert_tag_links(story_id: int, tag_id):
    return """
        INSERT OR IGNORE INTO tag_links (story_id, tag_id)
        VALUES ({0}, {1})
        """.format(story_id, tag_id)


def create_updates():
    return """
        CREATE TABLE IF NOT EXISTS updates (
            story_id            integer NOT NULL,
            timestamp           integer NOT NULL,
            words               integer NOT NULL,
            views               integer NOT NULL,
            thumbs_up           integer NOT NULL,
            thumbs_down         integer NOT NULL,
            approved_rank       integer,
            updated_rank        integer,
            heat_rank           integer,
            front_featured_rank integer,
            front_approved_rank integer,
            front_updated_rank  integer,
            front_heat_rank     integer,

            CONSTRAINT updates_story_id_fk FOREIGN KEY (story_id)
                REFERENCES stories (id),
            CONSTRAINT updates_pk PRIMARY KEY (story_id, timestamp)
        )"""


def insert_updates(story_id: int, timestamp: int, words: int, views: int, thumbs_up: int, thumbs_down: int, approved_rank: int = None, updated_rank: int = None, heat_rank: int = None,
                   front_featured_rank: int = None, front_approved_rank: int = None, front_updated_rank: int = None,
                   front_heat_rank: int = None):

    approved_rank = 'NULL' if approved_rank is None else approved_rank
    updated_rank = 'NULL' if updated_rank is None else updated_rank
    heat_rank = 'NULL' if heat_rank is None else heat_rank
    front_featured_rank = 'NULL' if front_featured_rank is None else front_featured_rank
    front_approved_rank = 'NULL' if front_approved_rank is None else front_approved_rank
    front_updated_rank = 'NULL' if front_updated_rank is None else front_updated_rank
    front_heat_rank = 'NULL' if front_heat_rank is None else front_heat_rank

    return """
        INSERT INTO updates (story_id, timestamp, words, views, thumbs_up, thumbs_down, approved_rank, 
        updated_rank, heat_rank, front_featured_rank, front_approved_rank, front_updated_rank, front_heat_rank)
        VALUES ({0}, {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10}, {11}, {12})
        """.format(story_id, timestamp, words, views, thumbs_up, thumbs_down, approved_rank,
                   updated_rank, heat_rank, front_featured_rank, front_approved_rank, front_updated_rank,
                   front_heat_rank)

