export type Bindings = {
  DB: D1Database;
};

export const getConnection = (c: any) => {
  return c.env.DB as D1Database;
};
