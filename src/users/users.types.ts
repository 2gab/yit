export type CreateUserRequest = {
  Body: {
    email: string;
    name?: string;
    password: string;
  };
};

export type UpdateUserRequest = {
  Params: {
    id: number;
  };

  Body: {
    name?: string;
  };
};

export type GetUserRequest = {
  Params: {
    id: number;
  };
};

export type DeleteUserRequest = {
  Params: {
    id: number;
  };
};