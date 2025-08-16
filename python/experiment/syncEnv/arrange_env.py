# import yaml
import argparse
import json
import re
import sys
from pathlib2 import Path


def main():
    # args = get_args()
    db = [{'schema': 'test', 'connectionSchema': 'ctest'}]
    s = Setting('dev', 'testApp', 'web', 'none', 'repos', 'msg', '200', db)
    p = json.dumps(s, cls=CustomEncoder)
    print(p)


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--app")
    parser.add_argument("--env")
    parser.add_argument("--repos")
    parser.add_argument("--checkout_dir")
    parser.add_argument("--message")
    parser.add_argument("--type")

    return parser.parse_args()


def get_yamls(root_dir):
    yamls = []
    target_folders = ['apps', 'lib/batch']
    for f in target_folders:
        target = Path(root_dir + '/' + f)
        yamls.extend(target.glob('database.yml'))



class Setting:
    def __init__(self, env, app, type, subApp, repo, message, limit, dbConnections):
        self.env = env
        self.app = app
        self.type = type
        self.subApp = subApp
        self.repo = repo
        self.message = message
        self.limit = limit
        self.__set_db_connections(dbConnections)

    def __set_db_connections(self, dbConnections):
        self.dbConnections = []
        for dbConnection in dbConnections:
            self.dbConnections.append(DbConnection(dbConnection['schema'], dbConnection['connectionSchema']))


class DbConnection:
    def __init__(self, schema, connectionSchema):
        self.schema = schema
        self.connectionSchema = connectionSchema


class CustomEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, Setting):
            return o.__dict__
        if isinstance(o, DbConnection):
            return o.__dict__
        return json.JSONEncoder.default(self, o)


if __name__ == "__main__":
    main()
