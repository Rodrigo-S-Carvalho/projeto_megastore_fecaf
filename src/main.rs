fn main() {
    let n1 = "A";
    let n2 =n1;
    let resultado;
    let resultado2;

        resultado = n1;
        resultado2 = n2;

    println!("{}", n1);
    println!("{}", n2);
    println!("{}", resultado);
    println!("{}", resultado2);
    imprimir(n2);    
}


fn imprimir(n3: &str) {
    println!("Função {}", n3);
}