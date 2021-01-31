from iuserrepository import IUserRepository
from user import User


class UserService():
    def __init__(self, user_repository: IUserRepository):
        self.user_repository = user_repository

    def exists(self, user: User):
        found = self.user_repository.Find(user.Name)

        return found != None

    def save(self, user: User):
        pass
