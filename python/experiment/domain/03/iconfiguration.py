from abc import ABCMeta, abstractclassmethod
from injector import Module


class IConfiguration(metaclass=ABCMetae):
    @abstractclassmethod
    def configure(self, Module):
        pass
