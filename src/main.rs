use std::io;
trait Displayable {
    fn show(&self);
}

struct Song {
    title: String,
    artist: String,
    album: String,
    year: i32,
}

impl Displayable for Song {
    fn show(&self) {
        println!(
            "Título: {}, Artista: {}, Álbum: {}, Año: {}",
            self.title, self.artist, self.album, self.year
        );
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T: Displayable> {
    head: Option<Box<Node<T>>>,
}

impl<T: Displayable> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            value: data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn show(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            node.value.show();
            current = node.next.as_ref();
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

fn main() {
    let mut song_list = LinkedList::new();

    loop {
        println!("Menú Principal:");
        println!("1. Agregar canción");
        println!("2. Mostrar canciones");
        println!("3. Salir");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error al leer la entrada");
        match option.trim() {
            "1" => {
                let mut title = String::new();
                let mut artist = String::new();
                let mut album = String::new();
                let mut year = String::new();

                println!("Introduce el título de la canción:");
                io::stdin()
                    .read_line(&mut title)
                    .expect("Error al leer el título");
                println!("Introduce el artista:");
                io::stdin()
                    .read_line(&mut artist)
                    .expect("Error al leer el artista");
                println!("Introduce el álbum:");
                io::stdin()
                    .read_line(&mut album)
                    .expect("Error al leer el álbum");
                println!("Introduce el año:");
                io::stdin()
                    .read_line(&mut year)
                    .expect("Error al leer el año");

                let year: i32 = year.trim().parse().expect("El año debe ser un número");

                let song = Song {
                    title: title.trim().to_string(),
                    artist: artist.trim().to_string(),
                    album: album.trim().to_string(),
                    year,
                };

                song_list.push(song);
            }
            "2" => {
                if song_list.is_empty() {
                    println!("No hay canciones en la lista.");
                } else {
                    song_list.show();
                }
            }
            "3" => {
                println!("Saliendo del programa.");
                break;
            }
            _ => println!("Opción no válida, intenta de nuevo."),
        }
    }
}
