use std::io;
use diesel::prelude::*;
use crate::db::establish_connection_pool;
use crate::model::{User, NewUser};
use crate::schema::users::dsl::users;
mod db;
mod model;
pub mod schema;

fn main() {
    loop {
        let mut pool = establish_connection_pool();
    let mut connection = pool.get().expect("erro ao pegar conexão do pool");

    let mut input = String::new();
    println!("Digite 1 para listar usuários, 2 para criar usuário, 3 para atualizar usuário:");
    io::stdin().read_line(&mut input).expect("erro ao ler entrada!");

    if input.trim() == "1" {
        let results = users.load::<User>(&mut connection).unwrap();
        println!("{:?}\n", results);
    }
    if input.trim() == "2" {
        println!("Digite o nome do usuário:");
        let mut nome = String::new();
        io::stdin().read_line(&mut nome).expect("erro ao ler nome");
        println!("Digite a senha do usuário:");
        let mut senha = String::new();
        io::stdin().read_line(&mut senha).expect("erro ao ler senha");

        let novo_usuario = NewUser {
            name: nome.trim(),
            password: senha.trim(),
        };
        diesel::insert_into(schema::users::table)
            .values(&novo_usuario)
            .execute(&mut connection)
            .expect("Erro ao inserir usuário");
        println!("Usuário criado com sucesso!");
    }
    if input.trim() == "3" {
        println!("Digite o ID do usuário para atualizar:");
        let mut id_input = String::new();
        io::stdin().read_line(&mut id_input).expect("erro ao ler ID");
        let user_id: i32 = id_input.trim().parse().expect("ID inválido");

        println!("Digite o novo nome do usuário:");
        let mut novo_nome = String::new();
        io::stdin().read_line(&mut novo_nome).expect("erro ao ler nome");
        println!("Digite a nova senha do usuário:");
        let mut nova_senha = String::new();
        io::stdin().read_line(&mut nova_senha).expect("erro ao ler senha");

        diesel::update(schema::users::table.find(user_id))
            .set((
                schema::users::name.eq(novo_nome.trim()),
                schema::users::password.eq(nova_senha.trim()),
            ))
            .execute(&mut connection)
            .expect("Erro ao atualizar usuário");
        println!("Usuário atualizado com sucesso!");
    }
    }
}
