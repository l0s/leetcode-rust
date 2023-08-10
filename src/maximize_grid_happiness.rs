// 1659. Maximize Grid Happiness
// https://leetcode.com/problems/maximize-grid-happiness/

pub struct Solution;

use std::{collections::HashSet, fmt::Display};

impl Solution {
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        let mut simulation = Simulation::new(
            m as usize,
            n as usize,
            introverts_count as u8,
            extroverts_count as u8,
        );
        let fittest = simulation.run();

        fittest.happiness
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Introvert,
    Extrovert,
}

impl Cell {
    fn baseline(&self) -> i32 {
        match self {
            Cell::Empty => 0,
            Cell::Introvert => 120,
            Cell::Extrovert => 40,
        }
    }
    fn neighbour_modifier(&self) -> i32 {
        match self {
            Cell::Empty => 0,
            Cell::Introvert => -30,
            Cell::Extrovert => 20,
        }
    }
}

/// For a given gene in which the parents have different alleles, choose a value
pub enum GeneSelectionStrategy {
    /// From the two chromosomes, choose the gene value that maximises happiness locally
    /// if there is no more room for that type of cell (allele), chose the other type available
    /// otherwise, make the cell empty
    OptimiseLocalHappiness,
    /// The gene value from the first parent will be preferred
    LeftParentDominant,
    /// The gene value from the happier parent will be preferred
    HappierParentDominant,
    /// Extroverts will take precedence
    ExtrovertDominant,
    /// Introverts will take precedence
    IntrovertDominant,
}

impl GeneSelectionStrategy {
    pub fn choose_cell_value(
        &self,
        lhs: &Grid,
        rhs: &Grid,
        introvert_count: u8,
        extrovert_count: u8,
        i: usize,
        j: usize,
    ) -> Cell {
        match self {
            GeneSelectionStrategy::OptimiseLocalHappiness => {
                if lhs.neighbour_happiness(i, j) > rhs.neighbour_happiness(i, j) {
                    // self maximises local happiness
                    match lhs.cells[i][j] {
                        Cell::Empty => Cell::Empty,
                        Cell::Introvert => {
                            if introvert_count < lhs.max_introverts {
                                Cell::Introvert
                            } else if rhs.cells[i][j] == Cell::Extrovert
                                && extrovert_count < lhs.max_extroverts
                            {
                                Cell::Extrovert
                            } else {
                                Cell::Empty
                            }
                        }
                        Cell::Extrovert => {
                            if extrovert_count < lhs.max_extroverts {
                                Cell::Extrovert
                            } else if rhs.cells[i][j] == Cell::Introvert
                                && introvert_count < lhs.max_introverts
                            {
                                Cell::Introvert
                            } else {
                                Cell::Empty
                            }
                        }
                    }
                } else {
                    // other maximises local happiness
                    match rhs.cells[i][j] {
                        Cell::Empty => Cell::Empty,
                        Cell::Introvert => {
                            if introvert_count < lhs.max_introverts {
                                Cell::Introvert
                            } else if lhs.cells[i][j] == Cell::Extrovert
                                && extrovert_count < lhs.max_extroverts
                            {
                                Cell::Extrovert
                            } else {
                                Cell::Empty
                            }
                        }
                        Cell::Extrovert => {
                            if extrovert_count < lhs.max_extroverts {
                                Cell::Extrovert
                            } else if lhs.cells[i][j] == Cell::Introvert
                                && introvert_count < lhs.max_introverts
                            {
                                Cell::Introvert
                            } else {
                                Cell::Empty
                            }
                        }
                    }
                }
            }
            GeneSelectionStrategy::LeftParentDominant => {
                let (preferred, backup) = (lhs, rhs);
                let (preferred, backup) = (preferred.cells[i][j], backup.cells[i][j]);
                if (preferred == Cell::Introvert && introvert_count < lhs.max_introverts)
                    || (preferred == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                    || preferred == Cell::Empty
                {
                    preferred
                } else if (backup == Cell::Introvert && introvert_count < lhs.introverts)
                    || (backup == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                {
                    backup
                } else {
                    Cell::Empty
                }
            }
            GeneSelectionStrategy::HappierParentDominant => {
                let (preferred, backup) = if lhs.happiness > rhs.happiness {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                let (preferred, backup) = (preferred.cells[i][j], backup.cells[i][j]);
                if (preferred == Cell::Introvert && introvert_count < lhs.max_introverts)
                    || (preferred == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                    || preferred == Cell::Empty
                {
                    preferred
                } else if (backup == Cell::Introvert && introvert_count < lhs.introverts)
                    || (backup == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                {
                    backup
                } else {
                    Cell::Empty
                }
            }
            GeneSelectionStrategy::ExtrovertDominant => {
                let (preferred, backup) = if lhs.cells[i][j] == Cell::Extrovert {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                let (preferred, backup) = (preferred.cells[i][j], backup.cells[i][j]);
                if (preferred == Cell::Introvert && introvert_count < lhs.max_introverts)
                    || (preferred == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                    || preferred == Cell::Empty
                {
                    preferred
                } else if (backup == Cell::Introvert && introvert_count < lhs.introverts)
                    || (backup == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                {
                    backup
                } else {
                    Cell::Empty
                }
            }
            GeneSelectionStrategy::IntrovertDominant => {
                let (preferred, backup) = if lhs.cells[i][j] == Cell::Introvert {
                    (lhs, rhs)
                } else {
                    (rhs, lhs)
                };
                let (preferred, backup) = (preferred.cells[i][j], backup.cells[i][j]);
                if (preferred == Cell::Introvert && introvert_count < lhs.max_introverts)
                    || (preferred == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                    || preferred == Cell::Empty
                {
                    preferred
                } else if (backup == Cell::Introvert && introvert_count < lhs.introverts)
                    || (backup == Cell::Extrovert && extrovert_count < lhs.max_extroverts)
                {
                    backup
                } else {
                    Cell::Empty
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    columns: usize,
    max_introverts: u8,
    max_extroverts: u8,
    happiness: i32,
    introverts: u8,
    extroverts: u8,
    empty_cells: usize,
    non_empty_cells: usize,
}

impl Grid {
    fn new(
        cells: Vec<Vec<Cell>>,
        rows: usize,
        columns: usize,
        max_introverts: u8,
        max_extroverts: u8,
    ) -> Self {
        let cell_happiness = |cell: &Cell, x: usize, y: usize| -> i32 {
            let neighbours = || -> Vec<(usize, usize)> {
                let mut result = Vec::with_capacity(4);
                if x > 0 {
                    result.push((x - 1, y));
                }
                if x < rows - 1 {
                    result.push((x + 1, y));
                }
                if y > 0 {
                    result.push((x, y - 1));
                }
                if y < columns - 1 {
                    result.push((x, y + 1));
                }
                result
            };
            let neighbour_modifier = cell.neighbour_modifier();
            let mut result = cell.baseline();
            for (x, y) in neighbours() {
                if cells[x][y] != Cell::Empty {
                    result += neighbour_modifier;
                }
            }
            result
        };

        let happiness = cells
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, cell)| cell_happiness(cell, i, j))
                    .sum::<i32>()
            })
            .sum();
        let mut introverts = 0;
        let mut extroverts = 0;
        let mut empty_cells = 0;
        for row in &cells {
            for cell in row {
                match cell {
                    Cell::Empty => empty_cells += 1,
                    Cell::Introvert => introverts += 1,
                    Cell::Extrovert => extroverts += 1,
                }
            }
        }
        let non_empty_cells = introverts as usize + extroverts as usize;
        Self {
            cells,
            rows,
            columns,
            max_introverts,
            max_extroverts,
            happiness,
            introverts,
            extroverts,
            empty_cells,
            non_empty_cells,
        }
    }
    pub fn random_grid(
        random: &mut Rand,
        rows: usize,
        columns: usize,
        max_introverts: u8,
        max_extroverts: u8,
    ) -> Self {
        // "You should decide how many people you want to live in the grid and assign each of them one grid cell.
        // Note that you do not have to have all the people living in the grid."
        let mut deck: Vec<Cell> =
            Vec::with_capacity(rows * columns + max_introverts as usize + max_extroverts as usize);
        deck.extend(vec![Cell::Introvert; max_introverts as usize]);
        deck.extend(vec![Cell::Extrovert; max_extroverts as usize]);
        deck.extend(vec![Cell::Empty; rows * columns]);

        let mut introvert_count = 0u8;
        let mut extrovert_count = 0u8;
        let mut cells = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(columns);
            for _ in 0..columns {
                let cell = deck.remove(random.rand_range(0, deck.len() - 1));
                match cell {
                    Cell::Introvert => introvert_count += 1,
                    Cell::Extrovert => extrovert_count += 1,
                    _ => (),
                }
                row.push(cell);
            }
            cells.push(row);
        }
        assert!(
            introvert_count <= max_introverts,
            "too many introverts in random grid"
        );
        assert!(
            extrovert_count <= max_extroverts,
            "too many extroverts in random grid"
        );

        Self::new(cells, rows, columns, max_introverts, max_extroverts)
    }

    pub fn cell_happiness(&self, i: usize, j: usize) -> i32 {
        let neighbour_modifier = self.cells[i][j].neighbour_modifier();
        let mut result = self.cells[i][j].baseline();
        for (x, y) in self.neighbours(i, j) {
            if self.cells[x][y] != Cell::Empty {
                result += neighbour_modifier;
            }
        }
        result
    }

    pub fn neighbour_happiness(&self, i: usize, j: usize) -> i32 {
        let mut result = 0;
        for (x, y) in self.neighbours(i, j) {
            result += self.cell_happiness(x, y);
        }
        result
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        if i > 0 {
            result.push((i - 1, j));
        }
        if i < self.cells.len() - 1 {
            result.push((i + 1, j));
        }
        if j > 0 {
            result.push((i, j - 1));
        }
        if j < self.cells[0].len() - 1 {
            result.push((i, j + 1));
        }
        result
    }

    pub fn combine(&self, other: &Self, random: &mut Rand) -> Self {
        let mut cells = (0..self.rows)
            .map(|_| vec![Cell::Empty; self.columns])
            .collect::<Vec<Vec<Cell>>>();
        let mut introvert_count = 0u8;
        let mut extrovert_count = 0u8;

        // the coördinates where the two chromosomes *do not* match
        let mut non_matching_indices = Vec::with_capacity(self.rows * self.columns);

        // where the chromosomes match exactly, propagate those values to the child
        for (i, my_row) in self.cells.iter().enumerate() {
            let other_row = &other.cells[i];
            for (j, my_cell) in my_row.iter().enumerate() {
                let other_cell = &other_row[j];
                if my_cell == other_cell {
                    cells[i][j] = *my_cell;
                    match my_cell {
                        Cell::Introvert => introvert_count += 1,
                        Cell::Extrovert => extrovert_count += 1,
                        _ => (),
                    }
                } else {
                    non_matching_indices.push((i, j));
                }
            }
        }
        random.shuffle(&mut non_matching_indices);

        let gene_selection_strategy = GeneSelectionStrategy::OptimiseLocalHappiness;
        // let gene_selection_strategy = GeneSelectionStrategy::LeftParentDominant;
        // let gene_selection_strategy = GeneSelectionStrategy::HappierParentDominant;
        // let gene_selection_strategy = GeneSelectionStrategy::ExtrovertDominant;
        // let gene_selection_strategy = GeneSelectionStrategy::IntrovertDominant;
        for (i, j) in non_matching_indices {
            if introvert_count >= self.max_introverts && extrovert_count >= self.max_extroverts {
                // exhausted all the introvert and extrovert slots
                // leave the remaining cells empty
                break;
            }

            let child = gene_selection_strategy.choose_cell_value(
                self,
                other,
                introvert_count,
                extrovert_count,
                i,
                j,
            );
            match child {
                Cell::Introvert => introvert_count += 1,
                Cell::Extrovert => extrovert_count += 1,
                _ => (),
            }
            cells[i][j] = child;
        }

        Self::new(
            cells,
            self.rows,
            self.columns,
            self.max_introverts,
            self.max_extroverts,
        )
    }

    pub fn mutate(&mut self, random: &mut Rand) {
        // pick a random mutation
        let mutations = vec![
            Mutation::AddIntrovert,
            Mutation::AddExtrovert,
            Mutation::Swap,
            Mutation::SetEmpty,
        ];
        let mut mutations = mutations
            .iter()
            .filter(|mutation| mutation.can_perform(self))
            .to_owned()
            .collect::<Vec<&Mutation>>();
        random.shuffle(&mut mutations);
        let mutation = mutations[0];
        // eprintln!("-- Applying mutation: {:?}", mutation);

        mutation.execute(self, random);
    }

    fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn is_mirror(&self, other: &Grid) -> bool {
        self.is_horizontal_mirror(other) || self.is_vertical_mirror(other)
    }

    fn is_horizontal_mirror(&self, other: &Grid) -> bool {
        for (i, my_row) in self.cells.iter().enumerate() {
            let other_row = &other.cells[i];
            for (j, my_cell) in my_row.iter().enumerate() {
                let other_cell = &other_row[self.columns - j - 1];
                if my_cell != other_cell {
                    return false;
                }
            }
        }
        true
    }

    fn is_vertical_mirror(&self, other: &Grid) -> bool {
        for (i, my_row) in self.cells.iter().enumerate() {
            let other_row = &other.cells[self.rows - i - 1];
            for (j, my_cell) in my_row.iter().enumerate() {
                let other_cell = &other_row[j];
                if my_cell != other_cell {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for row in &self.cells {
            for cell in row {
                let indicator = match cell {
                    Cell::Empty => '_',
                    Cell::Introvert => 'I',
                    Cell::Extrovert => 'E',
                };
                buffer.push(indicator);
            }
            buffer.push('\n');
        }
        write!(f, "{}", buffer)
    }
}

#[derive(Debug)]
enum Mutation {
    AddIntrovert,
    AddExtrovert,
    Swap,
    SetEmpty,
}

impl Mutation {
    fn can_perform(&self, grid: &Grid) -> bool {
        match self {
            Mutation::AddIntrovert => grid.introverts < grid.max_introverts,
            Mutation::AddExtrovert => grid.extroverts < grid.max_extroverts,
            Mutation::Swap => {
                grid.introverts as usize != grid.size()
                    && grid.extroverts as usize != grid.size()
                    && grid.empty_cells != grid.size()
            }
            Mutation::SetEmpty => grid.non_empty_cells > 0,
        }
    }

    fn execute(&self, grid: &mut Grid, random: &mut Rand) {
        match self {
            Mutation::AddIntrovert => {
                let indices = (0..grid.rows)
                    .flat_map(|x| (0..grid.columns).map(move |y| (x, y)))
                    .filter(|(x, y)| grid.cells[*x][*y] != Cell::Introvert)
                    .collect::<Vec<(usize, usize)>>();
                let (x, y) = indices[random.rand_range(0, indices.len() - 1)];
                grid.cells[x][y] = Cell::Introvert;
            }
            Mutation::AddExtrovert => {
                let indices = (0..grid.rows)
                    .flat_map(|x| (0..grid.columns).map(move |y| (x, y)))
                    .filter(|(x, y)| grid.cells[*x][*y] != Cell::Extrovert)
                    .collect::<Vec<(usize, usize)>>();
                let (x, y) = indices[random.rand_range(0, indices.len() - 1)];
                grid.cells[x][y] = Cell::Extrovert;
            }
            Mutation::Swap => {
                let (mut x1, mut y1) = (
                    random.rand_range(0, grid.rows - 1),
                    random.rand_range(0, grid.columns - 1),
                );
                let (mut x2, mut y2) = (
                    random.rand_range(0, grid.rows - 1),
                    random.rand_range(0, grid.columns - 1),
                );
                while grid.cells[x1][y1] == grid.cells[x2][y2] {
                    x1 = random.rand_range(0, grid.rows - 1);
                    y1 = random.rand_range(0, grid.columns - 1);
                    x2 = random.rand_range(0, grid.rows - 1);
                    y2 = random.rand_range(0, grid.columns - 1);
                }
                let temp = grid.cells[x1][y1];
                grid.cells[x1][y1] = grid.cells[x2][y2];
                grid.cells[x2][y2] = temp;
            }
            Mutation::SetEmpty => {
                let indices = (0..grid.rows)
                    .flat_map(|x| (0..grid.columns).map(move |y| (x, y)))
                    .filter(|(x, y)| grid.cells[*x][*y] != Cell::Empty)
                    .collect::<Vec<(usize, usize)>>();
                assert!(!indices.is_empty(), "No non-empty indices");
                let (x, y) = indices[random.rand_range(0, indices.len() - 1)];
                grid.cells[x][y] = Cell::Empty;
            }
        }
    }
}

/// https://github.com/d3spis3d/genetic-rust/blob/master/src/main.rs
pub struct Simulation {
    population: Vec<Grid>, // TODO store in sorted order?
    random: Rand,
    rows: usize,
    columns: usize,
    max_introverts: u8,
    max_extroverts: u8,
}

// TODO Generation struct

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p1\tp25\tp50\tp75\tp99\n{}\t{}\t{}\t{}\t{}",
            self.population[(0.01 * self.population.len() as f64) as usize].happiness,
            self.population[(0.25 * self.population.len() as f64) as usize].happiness,
            self.population[(0.50 * self.population.len() as f64) as usize].happiness,
            self.population[(0.75 * self.population.len() as f64) as usize].happiness,
            self.population[(0.99 * self.population.len() as f64) as usize].happiness
        )
    }
}

impl Simulation {
    pub fn new(rows: usize, columns: usize, max_introverts: u8, max_extroverts: u8) -> Self {
        let mut random = Rand::new(1337);
        let population_size = 768usize;
        let mut population = vec![];
        while population.len() < population_size {
            let grid =
                Grid::random_grid(&mut random, rows, columns, max_introverts, max_extroverts);
            if grid.happiness <= 0 {
                continue;
            }
            population.push(grid);
        }
        population.sort_unstable_by_key(|grid| grid.happiness);
        assert!(population[0].happiness > 0);

        Self {
            population,
            random,
            rows,
            columns,
            max_introverts,
            max_extroverts,
        }
    }

    pub fn run(&mut self) -> Grid {
        let debug = true;
        let possible_cell_values = if self.max_extroverts == 0 && self.max_introverts == 0 {
            1usize
        } else if self.max_introverts == 0 || self.max_extroverts == 0 {
            2
        } else {
            3
        };
        let phenotype = possible_cell_values.pow((self.rows * self.columns) as u32);

        // if the genotype fully covers the phenotype, don't evolve
        let max_generations = if self.population.len() >= phenotype {
            1
        } else {
            1024
        };
        let stopping_percentile = 0.35;
        let mut fittest = self.percentile(1.0);
        let mut population_fitness = self.percentile(stopping_percentile).happiness;
        let mut population_fitness_age = 0;
        for i in 0..max_generations {
            self.next_generation();
            let challenger = self.percentile(1.0);
            if challenger.happiness > fittest.happiness {
                fittest = challenger;
            }

            // stopping condition: population has stabilised
            let population_challenger = self.percentile(stopping_percentile);
            match population_challenger.happiness.cmp(&population_fitness) {
                std::cmp::Ordering::Less => population_fitness_age = 0,
                std::cmp::Ordering::Equal => population_fitness_age += 1,
                std::cmp::Ordering::Greater => {
                    population_fitness = population_challenger.happiness;
                    population_fitness_age = 0;
                }
            }
            if debug {
                eprintln!("-- generation {}:\n{}", i, self);
            }
            if population_fitness_age > 4 {
                break;
            }
        }
        if debug {
            eprintln!("-- grid discovered:\n{}", fittest);
        }
        fittest
    }

    fn next_generation(&mut self) {
        let mutation_rate = 0.006f64;
        let elite_count = 4;
        let weak_survivor_count = 64;
        let random_count = 64;
        let offspring_count =
            self.population.len() - elite_count - weak_survivor_count - random_count;

        let mut max_copies = 0;
        let mut parent_indices = Vec::new();
        fn compress(happiness: i32) -> usize {
            // happiness.log(2.7182818284590452353602874713527) as usize
            // happiness.log(1.618033988749) as usize
            // happiness.log(1.05) as usize
            (happiness as f64).sqrt().round() as usize
            // happiness as usize
        }
        for i in 0..self.population.len() {
            // TODO there is a more computationally efficient way to do weighted random selection
            let happiness = self.population[i].happiness;
            if happiness <= 0 {
                continue;
            }
            let copies = compress(happiness);
            max_copies = max_copies.max(copies);
            parent_indices.extend(vec![i; copies]);
        }
        let mut weak_survivor_indices = Vec::new();
        for i in 0..self.population.len() {
            let happiness = self.population[i].happiness;
            if happiness <= 0 {
                continue;
            }
            let copies = compress(happiness);
            let copies = max_copies - copies;
            weak_survivor_indices.extend(vec![i; copies]);
        }

        let mut result = Vec::with_capacity(self.population.len());
        // propagate the strongest chromosomes to the next generation
        result.extend_from_slice(&self.population[self.population.len() - elite_count..]);
        // combine the parents
        let mut encountered_pairs = HashSet::new();
        for _ in 0..offspring_count {
            let (mut x, mut y) = (
                parent_indices.remove(self.random.rand_range(0, parent_indices.len() - 1)),
                parent_indices.remove(self.random.rand_range(0, parent_indices.len() - 1)),
            );
            // oddly, ignoring pairs that are exactly the same or mirror images of each other produces worse results
            // perhaps due to the built-in mutation of the crossover process
            while x == y || encountered_pairs.contains(&(x, y))
            /*|| self.population[x] == self.population[y]
            || self.population[x].is_mirror(&self.population[y])*/
            {
                parent_indices.push(x);
                parent_indices.push(y);
                x = parent_indices.remove(self.random.rand_range(0, parent_indices.len() - 1));
                y = parent_indices.remove(self.random.rand_range(0, parent_indices.len() - 1));
            }
            // allow exactly one duplicate
            // even though we may have crossed this pair before, there is an element of mutation required to maintain the constraints
            encountered_pairs.insert((x, y));

            // crossover the two parents
            let child = self.population[x].combine(&self.population[y], &mut self.random);
            result.push(child);
        }
        // propagate some weak chromosomes to improve genetic diversity
        for _ in 0..weak_survivor_count {
            let survivor_index =
                weak_survivor_indices[self.random.rand_range(0, weak_survivor_indices.len() - 1)];
            let survivor = self.population[survivor_index].clone();
            result.push(survivor);
        }
        result.sort_unstable_by_key(|grid| grid.happiness);

        // mutate a random selection, but leave the elite untouched
        let target = (mutation_rate * 1000000f64) as usize;
        for i in 0..result.len() - elite_count {
            if self.random.rand_range(0, 1000000) < target {
                result[i].mutate(&mut self.random);
            }
        }

        // inject new chromosomes to improve genetic diversity
        for _ in 0..random_count {
            let grid = Grid::random_grid(
                &mut self.random,
                self.rows,
                self.columns,
                self.max_introverts,
                self.max_extroverts,
            );
            result.push(grid);
        }

        result.sort_unstable_by_key(|grid| grid.happiness); // re-sort to account for mutations
        assert_eq!(result.len(), self.population.len());

        self.population = result;
    }

    fn percentile(&self, percentile: f64) -> Grid {
        let index = (percentile * ((self.population.len() - 1) as f64)) as usize;
        self.population[index].clone()
    }
}

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

/// https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/11
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        self.w
    }

    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.is_empty() {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }

    pub fn rand_range(&mut self, a: usize, b: usize) -> usize {
        let m = b - a + 1;
        a + (self.rand() as usize % m)
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let m = 2;
        let n = 3;
        let introverts_count = 1;
        let extroverts_count = 2;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 240);
    }

    #[test]
    pub fn example2() {
        // given
        let m = 3;
        let n = 1;
        let introverts_count = 2;
        let extroverts_count = 1;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 260);
    }

    #[test]
    pub fn example3() {
        // given
        let m = 2;
        let n = 2;
        let introverts_count = 4;
        let extroverts_count = 0;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 240);
    }

    #[test]
    pub fn example14() {
        // given
        let m = 4;
        let n = 2;
        let introverts_count = 3;
        let extroverts_count = 4;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 590);
    }

    #[test]
    #[ignore]
    pub fn example16() {
        // given
        let m = 3;
        let n = 4;
        let introverts_count = 4;
        let extroverts_count = 3;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 680);
    }

    #[test]
    pub fn example29() {
        // given
        let m = 5;
        let n = 4;
        let introverts_count = 4;
        let extroverts_count = 3;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 680);
    }

    #[test]
    pub fn example31() {
        // given
        let m = 5;
        let n = 4;
        let introverts_count = 6;
        let extroverts_count = 3;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 920);
    }

    #[test]
    #[ignore]
    pub fn example33() {
        // given
        let m = 5;
        let n = 3;
        let introverts_count = 3;
        let extroverts_count = 6;

        // when
        let result = Solution::get_max_grid_happiness(m, n, introverts_count, extroverts_count);

        // then
        assert_eq!(result, 880);
    }
}
