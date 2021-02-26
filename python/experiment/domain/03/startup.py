from configuration import Configuration
from iconfiguration import IConfiguration


class Startup():
    def __init__(self, configuration: IConfiguration):
        self.configuration = configuration
