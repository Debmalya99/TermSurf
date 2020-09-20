// The index page for server4
print("Welcome to server 4!");
print("This server asks you to input an integer.");
print("You will get the square of the number as output");
fn show_menu(){
  print("MENU:");
  print("1. About");
  print("2. Find the square of a number");
  print("3. Exit");
  prompt("Enter your choice: ");
}
let client = new_client();
loop{
  show_menu();
  let choice = get_int();
  if choice == 1{
    print("Title: Squaring server");
    print("Purpose: Enter an integer and get its square as output");
    print("Codename: server4");
    print("Backend: Flask, python");
    print("Server author: Debmalya Mitra");
  }
  else if choice == 2{
    prompt("Please enter an integer: ");
    let num_str = getline();
    let url = "http://127.0.0.1:4000/"+num_str; // Our route URL
    let answer = client.get(url);
    print("Answer is "+answer);
  }
  else if choice == 3{
    break;
  }
  else{
    print("Wrong choice. Try again");
  }
}
