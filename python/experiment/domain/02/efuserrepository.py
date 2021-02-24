from iuserrepository import IUserRepository


class EFUserRepository(IUserRepository):
    def __init__(self, context: MyDbContext):
        self.context = context

    def find(self, name: UserName):
        pass
