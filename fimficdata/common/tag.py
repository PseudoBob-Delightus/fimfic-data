# Tag class
import re


class Tag:
    def __init__(self, data_tag_id, class_, data_tag, contents):
        pass
        self.id = data_tag_id
        self.type = re.search(r'tag-(.*)', class_).group(1)
        self.data = data_tag
        self.name = contents[0]
