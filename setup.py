"""
Setup script to perform the following tasks:
 - Create an empty login_metadata file if none exists
 - Create an autorun file
 - Create a test_all file
"""

if __name__ == "__main__":
    from os.path import exists
    from os import remove
    from sys import executable
    import re

    if not exists("login_metadata.json"):
        with open("login_metadata.json", "w") as login_metadata_file:
            login_metadata_file.write("""{
   "cookies":{

   },
   "headers":{

   }
}""")

    if exists("autorun.bat"):
        remove("autorun.bat")
    with open("autorun.bat", "w") as autorun_file:

        anaconda_dir = re.search(r'(.*?anaconda3)\\', executable).group(1)
        activate_file = anaconda_dir + "\\condabin\\activate.bat"
        if not exists(activate_file):
            raise FileNotFoundError("File not found: " + activate_file)

        # TODO: Find way to learn environment name dynamically
        #       Because: This will fail if the environment is not named fimfic-data
        # Reference for python module invocation: https://stackoverflow.com/a/40304201
        autorun_file.write("call {0} fimfic-data\n".format(activate_file))
        autorun_file.write("python -m fimficdata.scrape.scrape\n")
        autorun_file.write("timeout /t 300\n")

    if exists("test_all.bat"):
        remove("test_all.bat")
    with open("test_all.bat", "w") as pytest_file:

        anaconda_dir = re.search(r'(.*?anaconda3)\\', executable).group(1)
        activate_file = anaconda_dir + "\\condabin\\activate.bat"
        if not exists(activate_file):
            raise FileNotFoundError("File not found: " + activate_file)

        # TODO: Find way to learn environment name dynamically
        #       Because: This will fail if the environment is not named fimfic-data
        # Reference for python module invocation: https://stackoverflow.com/a/40304201
        pytest_file.write("call {0} fimfic-data\n".format(activate_file))
        pytest_file.write("del pytest.log\n")
        pytest_file.write("powershell \"python -m pytest -v | tee pytest.log\"\n")
        pytest_file.write("timeout /t 300\n")
