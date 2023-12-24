use crate::models::{
    Crate, NewCrate, NewRole, NewRustacean, NewUser, NewUserRole, Role, Rustacean, User, UserRole,
};
use crate::schema::{crates, roles, rustaceans, users, users_roles};
use diesel::{ExpressionMethods, QueryDsl, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(
        c: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).get_results(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c).await
    }
}

pub struct UserRepository;

impl UserRepository {
    pub async fn create(
        c: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user: User = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned(),
                    };

                    let role = RoleRepository::create(c, new_role).await?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)
                .await?;
        }

        Ok(user)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results(c)
            .await
    }

    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        roles::table
            .inner_join(users_roles::table)
            .filter(users_roles::user_id.eq(user.id))
            .select(roles::all_columns)
            .get_results(c)
            .await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }
}
