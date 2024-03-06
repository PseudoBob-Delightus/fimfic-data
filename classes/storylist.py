from classes.story import Story
from typing import Dict, List
from statistics import mean


class StoryList:
    def __init__(self):
        self.dict: Dict[int, Story] = {}
        self.usercounts: List[int] = []

    def add_story(self, story: Story):
        if story.id not in self.dict:
            self.dict[story.id] = story
        else:
            self.set_rank(story.id, story.approved_rank, story.updated_rank, story.heat_rank, story.front_featured_rank,
                          story.front_approved_rank, story.front_updated_rank, story.front_heat_rank)

    def add_usercount(self, usercount: int):
        # Add a usercount value to the object
        self.usercounts.append(usercount)

    def get_usercount(self):
        # Get the average usercount of the object
        return int(mean(self.usercounts))

    def set_totalhits(self, totalhits: int):
        # Set the total page hits in the update
        self.totalhits = totalhits

    def set_rank(self, id_: int, approved_rank: int = None, updated_rank: int = None, heat_rank: int = None,
                 front_featured_rank: int = None, front_approved_rank: int = None, front_updated_rank: int = None,
                 front_heat_rank: int = None):

        if id_ not in self.dict:
            raise KeyError('Story {} does not exist in the StoryList!'.format(id_))

        # Set the rank attributes of a story that already exists in the list
        if approved_rank is not None:
            self.dict[id_].approved_rank = approved_rank
        if updated_rank is not None:
            self.dict[id_].updated_rank = updated_rank
        if heat_rank is not None:
            self.dict[id_].heat_rank = heat_rank
        if front_featured_rank is not None:
            self.dict[id_].front_featured_rank = front_featured_rank
        if front_approved_rank is not None:
            self.dict[id_].front_approved_rank = front_approved_rank
        if front_updated_rank is not None:
            self.dict[id_].front_updated_rank = front_updated_rank
        if front_heat_rank is not None:
            self.dict[id_].front_heat_rank = front_heat_rank
