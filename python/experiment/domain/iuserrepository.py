from abc import ABC, ABCMeta, abstractclassmethod
from user import User
from username import UserName
from userid import UserId


class IUserRepository(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractclassmethod
    def save(self, user: User):
        pass

    @abstractclassmethod
    def find(self, name: UserName = None, id: UserId = None) -> User:
        pass

    @abstractclassmethod
    def delete(self, user: User):
        pass
