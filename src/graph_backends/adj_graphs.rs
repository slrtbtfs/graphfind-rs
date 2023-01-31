use std::{collections::HashMap, hash::Hash};

use petgraph::{visit::NodeRef, Direction::Incoming, graph::EdgeWeightsMut};

use crate::{graph::Graph, query::{PatternGraph, Matcher}};

use super::filter_map::FilterMap;
