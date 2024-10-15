use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreValidError {}

#[derive(Error, Debug)]
pub enum RearValidError {}


#[derive(Error, Debug)]
pub enum OpenError {}

#[derive(Error, Debug)]
pub enum CloseError {}
