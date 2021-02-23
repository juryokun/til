from iuserrepository import IUserRepository
from user import User
from username import UserName


class InMemoryUserRepository(IUserRepository):

    def __init__(self):
        test_user = User(UserName("yamada"))
        self.store = {test_user.id: test_user}

    def save(self, user: User):
        self.store[user.id] = user

    def find(self, name: UserName) -> User:
        for i in self.store.values():
            if i.name == name:
                return i
        return None

    def delete(self, user: User):
        pass

    def exists(self, user: User) -> bool:
        return user in self.store.values()
