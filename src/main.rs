mod db;
mod model;
pub mod schema;

use std::io;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::model::{NewUser, User};
use crate::db::establish_connection;

fn main() {
    let mut connection = establish_connection();

    println!("Selecione uma operação do banco de dados!");
    println!("1 - Listar todos os usuários do banco de dados");
    println!("2 - Criar um novo usuário no banco de dados");
    println!("3 - Atualizar um usuário no banco de dados");
    println!("4 - Deletar um usuário no banco de dados");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    if input.trim() == "1" {
        let results = users.load::<User>(&mut connection).expect("Erro ao carregar usuários");

        for user in results {
            println!("Nome: {}, Email: {}", user.name, user.email);
        }
    }

    if input.trim() == "2" {
        let mut name_input = String::new();
        let mut email_input = String::new();
        let mut password_input = String::new();

        println!("Digite o nome:");
        io::stdin().read_line(&mut name_input).unwrap();

        println!("Digite o email:");
        io::stdin().read_line(&mut email_input).unwrap();

        println!("Digite a senha:");
        io::stdin().read_line(&mut password_input).unwrap();

        let new_user = NewUser {
            name: name_input.trim(),
            email: email_input.trim(),
            password: password_input.trim(),
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut connection)
            .expect("Falha ao salvar o novo usuário");
    }

    if input.trim() == "3" {
        let mut email_str = String::new();
        println!("Digite o email do usuário a atualizar:");
        io::stdin().read_line(&mut email_str).unwrap();

        let mut new_name_input = String::new();
        println!("Digite o novo nome:");
        io::stdin().read_line(&mut new_name_input).unwrap();

        diesel::update(users.filter(email.eq(email_str.trim())))
            .set(name.eq(new_name_input.trim()))
            .execute(&mut connection)
            .expect("Erro ao atualizar usuário");
    }

    if input.trim() == "4" {
        let mut email_str = String::new();
        println!("Digite o email do usuário a deletar:");
        io::stdin().read_line(&mut email_str).unwrap();

        diesel::delete(users.filter(email.eq(email_str.trim())))
            .execute(&mut connection)
            .expect("Erro ao deletar usuário");
    }
}
