import os
import datetime

import http.client
from urllib.parse import urlparse

input_url = "https://adventofcode.com/{year}/day/{day}/input"

# Contains the session cookie. (Open advent of code page in dev tools)
session = open("session.txt", 'r').read().strip()


def get(url, cookies_dict):
    parsed_url = urlparse(url)
    
    connection = http.client.HTTPSConnection(parsed_url.netloc)
    
    cookies = "; ".join(f"{key}={value}" for key, value in cookies_dict.items())
    headers = { "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)", "Cookie": cookies }
    connection.request("GET", parsed_url.path, headers=headers)
    response = connection.getresponse()
    data = response.read().decode('utf-8')
    return data

def main(argc, argv):
    # print("Usage: python3 setup.py <day> <year>")

    # Get the current datetime
    today = datetime.datetime.now()
    url = input_url.format(day=today.day if argc < 1 else int(argv[0]), year=today.year)
    print(url)

    cookies = { "session": session }
    response = get(url, cookies)

    current_directory = os.getcwd()
    day = today.day if argc < 1 else int(argv[0])
    day_str = "Day" + str(day).zfill(2) 
    day_directory = os.path.join(current_directory, day_str) 

    if not os.path.exists(day_directory):
        os.mkdir(day_directory)
        print("Created directory: " + day_directory)
    else:
        print("Directory already exists: " + day_directory)

    input_file = os.path.join(day_directory, "input.txt")
    if not os.path.exists(input_file):
        with open(input_file, 'w') as f:
            f.write(response)
        print("- Created file: " + input_file)

    with open("Template/Template.cs", 'r') as f:
        template = f.read().replace("Template", day_str)
        file_name = os.path.join(day_directory, day_str + ".cs")
        with open(os.path.join(day_directory, file_name), 'w') as f:
            f.write(template)
        print("- Created file: " + file_name)
            

if __name__ == "__main__":
    import sys
    args = sys.argv[1:]
    main(len(args), args)

