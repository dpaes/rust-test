// Importa um modulo(pacote) interno, seja por .rs ou folder com esse nome
mod utils;

// Usa crates(bibliotecas) externas
use std::{collections::HashMap, hash::Hash};
use firebase_rs::*;
use serde::{Deserialize, Serialize};

// usa o modulo e funções especificas dele
use utils::functions::{string_to_response, string_to_user};

// Definido uma struct(semelhante a uma classe) para User
// #[derive...] é uma macro de atributo em que permite adicionar automaticamente
// traits(interfaces) para as strucs ou enums, ou seja adicionar métodos as estruturas de dados
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// Definido uma struct para Response
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

// #[tokio::main] é uma macro que simplifica o uso de async e config do runtime Tokio para rust
#[tokio::main]
async fn main() {
    let user = User {
        name: "nome do usuario".to_string(),
        age: 18,
        email: "email@doUsuario.com".to_string(),
    };

    /* 

    Em Rust, o método unwrap() é usado para extrair o valor de um tipo Result ou Option. Esses tipos são usados para representar operações que podem falhar ou retornar um valor nulo, respectivamente.

    Por que usar unwrap()?

    Simplicidade: Em situações onde você tem certeza de que a operação não falhará, unwrap() oferece uma maneira concisa de acessar o valor resultante.
    Tratamento de erros explícito: Ao usar unwrap(), você está explicitamente dizendo ao compilador que espera que a operação seja bem-sucedida. Se a operação falhar, o programa entrará em pânico, o que pode ser útil para identificar e corrigir erros durante o desenvolvimento.
    Para que serve unwrap()?

    Result<T, E>: Quando aplicado a um Result, unwrap() retorna o valor T se a operação foi bem-sucedida. Se a operação falhou, o programa entra em pânico com a mensagem de erro E.
    Option<T>: Quando aplicado a um Option, unwrap() retorna o valor T se ele estiver presente. Se o valor for None, o programa entra em pânico.

    */
    let firebase = Firebase::new("your-url-from-firebase").unwrap();

    /* 
    Em Rust, o símbolo & antes de uma variável indica uma referência. Referências são como "apelidos" ou "ponteiros" para um valor na memória, permitindo que você acesse e manipule dados sem precisar copiá-los.

    Existem dois principais motivos para usar referências em Rust:

    1. Eficiência:

    Evita cópias desnecessárias: Ao passar uma referência para uma função, você evita a cópia de grandes quantidades de dados, o que melhora o desempenho, especialmente para tipos de dados complexos ou grandes.
    Permite modificação in-place: Referências mutáveis (&mut) permitem que você altere o valor original da variável sem precisar retornar um novo valor.
    2. Propriedade e Tempo de Vida (Borrowing and Lifetimes):

    Gerenciamento de memória seguro: Rust usa um sistema de propriedade e tempo de vida para garantir que as referências sempre apontem para dados válidos. Isso previne erros comuns como ponteiros nulos ou dangling pointers.
    Compartilhamento de dados: Referências permitem que várias partes do código acessem os mesmos dados sem criar cópias, facilitando o compartilhamento de informações.
    */
    let response = set_user(&firebase, &user).await;
    
    // necessário incluir "mut" para a variável ser modificada, do contrario seria uma variavel imutável (read only)
    // variáveis mutáveis ficam com um sublinhado abaixo deles no código...
    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);
    
    // O await após uma chamada de função assíncrona é usado para esperar a conclusão da operação assíncrona antes de continuar a execução do programa.
    let users = get_users(&firebase).await;
    println!("{:?}", users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    // println! é uma macro, que ao usar "{}" ela insere o valor da variável dentro da string
    // os ":" separam o placeholder da especificação de formatação (é possível criar uma formataçao na saida)
    // quando utilizado ":?" especifica que deve sair da forma mais conveniente possivel no contexto da variável
    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted");
}

// Sempre que tiver algo após os parenteses e tiver "->", indica o tipo de valor retornado, caso não tenha, é uma função sem retorno
async fn set_user(firebase_client: &Firebase, user: &User) -> Response{
    let firebase = firebase_client.at("users");
    let _user = firebase.set::<User>(&user).await;
    return string_to_response(&_user.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}",users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users");
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}