from iuserrepository import IUserRepository
from userservice import UserService
from user import User
from userid import UserId
from username import UserName
from userupdatecommand import UserUpdateCommand
from injector import inject


class UserApplicationService():
    @inject
    def __init__(self, user_repository: IUserRepository, user_service: UserService):
        self._user_repository = user_repository
        self._user_service = user_service

    def register(self, name: str):
        user = User(UserName(name))
        if self._user_service.exists(user):
            raise ValueError("既にユーザが存在します")

        self._user_repository.save(user)

    def get(self, user_id: str):
        target_id = UserId(user_id)
        user = self._user_repository.find(target_id)
        return user

    def update(self, command: UserUpdateCommand):
        target_id = UserId(command.id)
        user = self._user_repository.find(target_id)
        if user == None:
            raise ValueError("ユーザが存在しません")

        name = command.name
        if name != None:
            new_user_name = UserName(name)
            user.change_name(new_user_name)
            if self._user_service.exists(user):
                raise ValueError("ユーザが既に存在しています")

        self._user_repository.save(user)
