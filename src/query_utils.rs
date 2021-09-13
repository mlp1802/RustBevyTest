
use crate::model::*;
use bevy::{prelude::*, render::render_graph::Command};
pub fn lol<A,B,C> (x:A,y:B)->A{
    x
}
pub fn first_value<F,A,T>(query: &Query<&A>, f: F) -> Option<T>
where
    A: std::marker::Sync,
    A: std::marker::Send,
    A: 'static,
    F: Fn(&A) ->  T,
{
    for x in query.iter() {
        return Some(f(x));
        //return None;//return Some(f(x));
    }
    None
}
pub fn with_first_value<F,A,T>(query: &Query<&A>, f: F)
where
    A: std::marker::Sync,
    A: std::marker::Send,
    A: 'static,
    F: Fn(&A)
{
    for x in query.iter() {
        f(x)
    }
}
