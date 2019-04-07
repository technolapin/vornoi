use rand::random;
use std::cmp;

const W: usize = 1600;
const H: usize = 1000;


type Position = (usize, usize);


fn randpos() -> (usize, usize)
{
    (rand::random::<usize>() % W,
     rand::random::<usize>() % H)
}

fn random_seeds(n: usize) -> Vec<(usize, usize)>
{
    match n
    {
        0 => Vec::new(),
        _ =>
        {
            let mut v = random_seeds(n-1);
            v.push(randpos());
            v
        }
    }
}

fn voisins ((x, y): Position) -> Vec<Position>
{
    let mut v = Vec::with_capacity(8);
    for i in (cmp::max(1, x)-1)..(cmp::min(W-2, x)+2)
    {
        for j in (cmp::max(1, y)-1)..(cmp::min(H-2, y)+2)
        {
            if (i, j) != (x, y)
            {
                v.push((i, j));
            }
        }
    }
    v
}

    
fn iterate(
    front: &mut Vec<Position>,
    mat: &mut [[i32; H]]
)
{
    let mut next_front = Vec::new();
    for (i0, j0) in front.iter()
    {
        for (i, j) in voisins((*i0, *j0)).iter()
        {
            if mat[*i][*j] == -1
            {
                next_front.push((*i, *j));
                mat[*i][*j] = mat[*i0][*j0];
            }
        }
    }
    *front = next_front;
}


fn convpix(n: i32) -> image::Rgb<u8>
{
    let b = n % 256;
    let g = (n / 256) % 256;
    let r =  n / 65536;
    image::Rgb([r as u8, g as u8, b as u8])
}


fn make_image(mat: &[[i32; H]], name: String)
{
    let mut img = image::RgbImage::new(W as u32, H as u32);
    
    for i in 0..W
    {
        for j in 0..H
        {
            img.put_pixel(
                i as u32,
                j as u32,
                convpix(mat[i][j])
            );
        }
    }
    

    img.save(name);

}


fn main() {



    let mut mat = [[-1 as i32; H]; W];

    let mut front = random_seeds(64);

    for (i, j) in front.iter()
    {
        mat[*i][*j] = rand::random::<i32>();
        println!("coul {}", mat[*i][*j]);
    }

    let mut n = 0;
    while front.len() != 0
    {
        iterate(&mut front, &mut mat);
        make_image(
            &mat,
            //format!("images/frame_{:03}.png", n)
            //to generate the gif
            //(the slowness of my sys calls makes it totaly useless to optimize
        );
        n += 1;
        println!("{:?}", front.len());
    }

    make_image(&mat, format!("final.png"));
    
    
}
