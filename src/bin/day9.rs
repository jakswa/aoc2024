fn main() {
    //let input = SAMPLE;
    let input = aoc2024::input("9");
    let disk_pairs = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    let disk = disk_pairs
        .chunks(2)
        .map(|ab| (ab[0], *ab.get(1).unwrap_or(&0)))
        .collect::<Vec<(u8, u8)>>();
    part1(&disk);
    part2(disk);
}

fn part2(mut disk: Vec<(u8, u8)>) {
    let mut to_insert = Vec::new();
    let mut to_remove = Vec::new();
    let mut idisk: Vec<(usize, u8, u8)> = disk
        .iter()
        .enumerate()
        .map(|(ind, ab)| (ind, ab.0, ab.1))
        .collect();
    idisk
        .clone()
        .iter()
        .rev()
        .for_each(|(id, filesize, freespace)| {
            let (curr_ind, _) = idisk
                .iter()
                .enumerate()
                .find(|(ind, d)| d.0 == *id)
                .unwrap();
            if let Some(hole) = idisk
                .iter_mut()
                .enumerate()
                .filter(|(index, (_, _, _))| *index < curr_ind)
                .find(|(_index, (_did, _dfs, dspace))| dspace >= filesize)
            {
                to_remove.push(*id);
                to_insert.push((hole.0 + 1, *id, *filesize, hole.1 .2 - filesize));
            }
            to_remove.drain(..).for_each(|id| {
                let res = idisk
                    .iter()
                    .enumerate()
                    .find(|(_ind, d)| d.0 as usize == id);
                if let Some((ind, d)) = res {
                    if ind > 0 {
                        idisk[ind - 1].2 += d.1 + d.2;
                    }
                    idisk.remove(ind);
                }
            });
            to_insert.drain(..).for_each(|u| {
                idisk[u.0 - 1].2 = 0;
                idisk.insert(u.0, (u.1, u.2, u.3));
            });
        });

    let mut cursor = 0;
    let mut sum = 0;
    idisk.iter().for_each(|(file_id, filesize, freespace)| {
        (0..*filesize).for_each(|_i| {
            sum += cursor * file_id;
            cursor += 1
        });
        cursor += *freespace as usize;
    });
    println!("part2: {sum}");
}

fn part1(disk: &Vec<(u8, u8)>) {
    let mut diskend = disk.iter().enumerate().rev();
    let mut rentry: (usize, (u8, u8)) = diskend.next().map(|(i, j)| (i, j.clone())).unwrap();
    let mut squished: Vec<(usize, u8)> = Vec::new();
    disk.iter()
        .enumerate()
        .for_each(|(id, (filesize, mut freesize))| {
            if id > rentry.0 {
                return;
            }
            if rentry.0 == id {
                squished.push((id, rentry.1 .0));
                return;
            }
            if id != rentry.0 {
                squished.push((id, *filesize));
            }
            loop {
                let insert_size = freesize.min(rentry.1 .0);
                squished.push((rentry.0, insert_size));
                freesize -= insert_size;
                rentry.1 = (rentry.1 .0 - insert_size, rentry.1 .1);
                if rentry.1 .0 == 0 {
                    rentry = diskend.next().map(|(i, j)| (i, j.clone())).unwrap();
                }
                if freesize == 0 || id >= rentry.0 {
                    break;
                }
            }
        });

    let mut cursor = 0;
    let mut sum: usize = 0;
    squished.iter().for_each(|(file, size)| {
        (0..*size).for_each(|_i| {
            sum += cursor * file;
            cursor += 1
        });
    });
    println!("part1: {sum}");
}

const SAMPLE: &str = "2333133121414131402";
