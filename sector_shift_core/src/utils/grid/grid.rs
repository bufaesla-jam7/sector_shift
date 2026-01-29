use core::slice;
use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::grid::PointIterRowMajor;

#[derive(Serialize, Deserialize, Reflect, bitcode::Encode, bitcode::Decode, Debug, Clone)]
pub struct Grid<T> {
    size: (u32, u32),
    data: Vec<T>,
}

// Constructors
impl<T> Grid<T> {
    /// Create a new `Grid` from a `(width, height)` and `Vec<T>`
    pub fn new(size: (u32, u32), data: Vec<T>) -> Self {
        assert_eq!(
            (size.0 * size.1) as usize,
            data.len(),
            "Data length does not match grid size"
        );
        Self { size, data }
    }

    /// Create a new `Grid` from a `(width, height)` cloning the value
    pub fn new_clone(size: (u32, u32), value: T) -> Self
    where
        T: Clone,
    {
        let capacity = (size.0 * size.1) as usize;

        Self::new(size, vec![value; capacity])
    }

    /// Create a new `Grid` from a `(width, height)` copying the value
    pub fn new_copy(size: (u32, u32), value: T) -> Self
    where
        T: Copy,
    {
        let capacity = (size.0 * size.1) as usize;
        // let data = vec![T::default; capacity]; // T not necessarily Clone
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(value);
        }

        Self::new(size, data)
    }

    /// Create a new `Grid` from a `(width, height)` cloning the default value
    pub fn new_default(size: (u32, u32)) -> Self
    where
        T: Default + Clone,
    {
        let capacity = (size.0 * size.1) as usize;

        Self::new(size, vec![T::default(); capacity])
    }

    /// Create a new `Grid` from a `(width, height)` obtaining a new default value for all data
    pub fn new_default_no_clone(size: (u32, u32)) -> Self
    where
        T: Default,
    {
        let capacity = (size.0 * size.1) as usize;
        // let data = vec![T::default; capacity]; // T not necessarily Clone
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            data.push(T::default());
        }

        Self::new(size, data)
    }

    /// Create a new `Grid` from a `(width, height)` obtaining a value from a `Fn(index,
    /// position) -> T`
    pub fn new_fn(size: (u32, u32), mut f: impl FnMut(usize, (i32, i32)) -> T) -> Self {
        let capacity = (size.0 * size.1) as usize;
        let mut data = Vec::with_capacity(capacity);
        let mut i = 0;
        for y in 0..size.1 as i32 {
            for x in 0..size.0 as i32 {
                data.push(f(i, (x, y)));
                i += 1;
            }
        }

        Self::new(size, data)
    }
}

// Properties
impl<T> Grid<T> {
    /// Obtain the size of this `Grid`
    #[inline]
    pub const fn size(&self) -> (u32, u32) {
        self.size
    }

    /// Obtain the width of this `Grid`
    #[inline]
    pub const fn width(&self) -> u32 {
        self.size.0
    }

    /// Obtain the height of this `Grid`
    #[inline]
    pub const fn height(&self) -> u32 {
        self.size.1
    }

    /// Determine if a position is inside of this `Grid`
    #[inline]
    pub const fn in_bounds(&self, position: (i32, i32)) -> bool {
        position.0 >= 0
            && position.0 < self.width() as i32
            && position.1 >= 0
            && position.1 < self.height() as i32
    }

    /// Determine if a position is on the border of this `Grid`
    pub const fn on_border(&self, position: (i32, i32)) -> bool {
        self.in_bounds(position)
            && (position.0 == 0
                || position.0 == self.width() as i32 - 1
                || position.1 == 0
                || position.1 == self.height() as i32 - 1)
    }

    /// Determine if an index is valid in this `Grid`
    ///
    /// NOTE: A position converted to an index may not be `in_bounds` yet still pass
    /// `is_valid`. Given a `Grid` with size (3, 3), a position (0, 4) is not inside this
    /// `Grid` but provides a valid index.
    #[inline]
    pub fn is_valid_index(&self, index: usize) -> bool {
        index < self.data.len()
    }

    /// Determine if an index is on the border of this `Grid`
    pub fn is_border_index(&self, index: usize) -> bool {
        let position = self.index_to_position_unchecked(index);
        self.on_border(position)
    }
}

// Index/Position Conversion
impl<T> Grid<T> {
    /// Converts a position into an index
    pub const fn position_to_index(&self, position: (i32, i32)) -> Option<usize> {
        if self.in_bounds(position) { Some(self.position_to_index_unchecked(position)) } else { None }
    }

    /// Converts a position into an index
    #[inline]
    pub const fn position_to_index_unchecked(&self, position: (i32, i32)) -> usize {
        (position.1 * self.width() as i32 + position.0) as usize
    }

    /// Converts an index into a position
    pub const fn index_to_position(&self, index: usize) -> Option<(i32, i32)> {
        let position = self.index_to_position_unchecked(index);
        if self.in_bounds(position) { Some(position) } else { None }
    }

    /// Converts an index into a position
    #[inline]
    pub const fn index_to_position_unchecked(&self, index: usize) -> (i32, i32) {
        (
            (index % self.width() as usize) as i32,
            (index / self.width() as usize) as i32,
        )
    }
}

// Accessors
impl<T> Grid<T> {
    /// Borrow the full `Grid`
    #[inline]
    pub const fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Mutably borrow the full `Grid`
    #[inline]
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Borrow a value at an index
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Mutably borrow a value at an index
    #[inline]
    pub fn get_mut_index(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Borrow a value at a position
    pub fn get(&self, position: (i32, i32)) -> Option<&T> {
        if self.in_bounds(position) {
            self.get_index(self.position_to_index_unchecked(position))
        } else {
            None
        }
    }

    /// Mutably borrow a value at a position
    pub fn get_mut(&mut self, position: (i32, i32)) -> Option<&mut T> {
        if self.in_bounds(position) {
            self.get_mut_index(self.position_to_index_unchecked(position))
        } else {
            None
        }
    }

    /// Take the value at a position leaving default
    pub fn take(&mut self, position: (i32, i32)) -> Option<T>
    where
        T: Default,
    {
        self.get_mut(position).map(|dest| std::mem::take(dest))
    }

    /// Replace the value at a position leaving src
    pub fn replace(&mut self, position: (i32, i32), src: T) -> Option<T> {
        self.get_mut(position).map(|dest| std::mem::replace(dest, src))
    }

    /// Swap the value at a position with another value
    pub fn swap(&mut self, position: (i32, i32), src: &mut T) {
        if let Some(dest) = self.get_mut(position) {
            std::mem::swap(dest, src);
        }
    }
}

// Helpers
impl<T> Grid<T> {
    pub fn blit(
        destination: &mut Self,
        to_offset: (i32, i32),
        size: (u32, u32),
        source: &Self,
        from_offset: (i32, i32),
    ) where
        T: Clone,
    {
        for y in 0..size.1 as i32 {
            let destination_y = y + to_offset.1;
            let source_y = y + from_offset.1;
            for x in 0..size.0 as i32 {
                let destination_x = x + to_offset.0;
                let source_x = x + from_offset.0;

                if let Some(source_value) = source.get((source_x, source_y))
                    && let Some(destination_value) = destination.get_mut((destination_x, destination_y))
                {
                    *destination_value = source_value.clone();
                }
            }
        }
    }

    pub fn neighbors(&self, position: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        (-1..=1)
            .flat_map(move |dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(move |&(dx, dy)| dx != 0 || dy != 0)
            .map(move |(dx, dy)| (position.0 + dx, position.1 + dy))
            .filter(move |&pos| self.in_bounds(pos))
    }
}

// Iterators
impl<T> Grid<T> {
    pub fn iter(&'_ self) -> slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&'_ mut self) -> slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = ((i32, i32), &T)> {
        let width = self.width() as usize;
        self.data.iter().enumerate().map(move |(index, value)| {
            let x = (index % width) as i32;
            let y = (index / width) as i32;
            ((x, y), value)
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = ((i32, i32), &mut T)> {
        let width = self.width() as usize;
        self.data.iter_mut().enumerate().map(move |(index, value)| {
            let x = (index % width) as i32;
            let y = (index / width) as i32;
            ((x, y), value)
        })
    }

    pub fn position_iter(&self) -> PointIterRowMajor {
        PointIterRowMajor::new(self.size)
    }

    pub fn row(&'_ self, y: usize) -> Option<slice::Iter<'_, T>> {
        if y < self.size.1 as usize {
            let start = y * self.size.0 as usize;
            Some(self.data[start..start + self.size.0 as usize].iter())
        } else {
            None
        }
    }

    pub fn row_mut(&'_ mut self, y: usize) -> Option<slice::IterMut<'_, T>> {
        if y < self.size.1 as usize {
            let start = y * self.size.0 as usize;
            Some(self.data[start..start + self.size.0 as usize].iter_mut())
        } else {
            None
        }
    }

    pub fn rows(&'_ self) -> slice::ChunksExact<'_, T> {
        self.data.chunks_exact(self.size.0 as usize)
    }

    pub fn rows_mut(&'_ mut self) -> slice::ChunksExactMut<'_, T> {
        self.data.chunks_exact_mut(self.size.0 as usize)
    }

    pub fn column(&'_ self, x: usize) -> Option<std::iter::StepBy<slice::Iter<'_, T>>> {
        if x < self.size.0 as usize {
            Some(self.data[x..].iter().step_by(self.size.0 as usize))
        } else {
            None
        }
    }

    pub fn column_mut(&'_ mut self, x: usize) -> Option<std::iter::StepBy<slice::IterMut<'_, T>>> {
        if x < self.size.0 as usize {
            Some(self.data[x..].iter_mut().step_by(self.size.0 as usize))
        } else {
            None
        }
    }

    pub fn columns(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.size.0 as usize).map(|column_index| {
            (0..self.size.1 as usize)
                .map(|row_index| &self.data[row_index * self.size.0 as usize + column_index])
                .collect::<Vec<&T>>()
        })
    }

    pub fn columns_mut(&mut self) -> impl Iterator<Item = Vec<&mut T>> {
        (0..self.size.0 as usize).map(|column_index| {
            (0..self.size.1 as usize)
                .map(|row_index| {
                    // Create a raw pointer to each position in the column bypassing borrow checking temporarily
                    &mut self.data[row_index * self.size.0 as usize + column_index] as *mut T
                })
                .collect::<Vec<*mut T>>()
                .into_iter()
                // SAFETY: Safe because each raw pointer is pointing to a different place in the `Grid` from above.
                .map(|ptr| unsafe { &mut *ptr })
                .collect::<Vec<&mut T>>()
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.get(index).expect("Invalid index position")
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.get_mut(index).expect("Invalid index position")
    }
}

impl<T> Index<(u32, u32)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let index = (index.0 as i32, index.1 as i32);
        self.get(index).expect("Invalid index position")
    }
}

impl<T> IndexMut<(u32, u32)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let index = (index.0 as i32, index.1 as i32);
        self.get_mut(index).expect("Invalid index position")
    }
}
