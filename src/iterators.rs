// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use std::convert::TryInto;

use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::exceptions::{PyIndexError, PyNotImplementedError};
use pyo3::prelude::*;
use pyo3::types::PySequence;

/// A custom class for the return from :func:`retworkx.bfs_successors`
///
/// This class is a container class for the results of the
/// :func:`retworkx.bfs_successors` function. It implements the Python
/// sequence protocol. So you can treat the return as read-only
/// sequence/list that is integer indexed. If you want to use it as an
/// iterator you can by wrapping it in an ``iter()`` that will yield the
/// results in order.
///
/// For example::
///
///     import retworkx
///
///     graph = retworkx.generators.directed_path_graph(5)
///     bfs_succ = retworkx.bfs_successors(0)
///     # Index based access
///     third_element = bfs_succ[2]
///     # Use as iterator
///     bfs_iter = iter(bfs_succ)
///     first_element = next(bfs_iter)
///     second_element = nex(bfs_iter)
///
#[pyclass(module = "retworkx")]
pub struct BFSSuccessors {
    pub bfs_successors: Vec<(PyObject, Vec<PyObject>)>,
    pub index: usize,
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for BFSSuccessors {
    fn __richcmp__(
        &self,
        other: &'p PySequence,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<bool> {
        let compare = |other: &PySequence| -> PyResult<bool> {
            if other.len()? as usize != self.bfs_successors.len() {
                return Ok(false);
            }
            let gil = Python::acquire_gil();
            let py = gil.python();
            for i in 0..self.bfs_successors.len() {
                let other_raw = other.get_item(i.try_into().unwrap())?;
                let other_value: (PyObject, Vec<PyObject>) =
                    other_raw.extract()?;
                if self.bfs_successors[i].0.as_ref(py).compare(other_value.0)?
                    != std::cmp::Ordering::Equal
                {
                    return Ok(false);
                }
                for (index, obj) in self.bfs_successors[i].1.iter().enumerate()
                {
                    if obj.as_ref(py).compare(&other_value.1[index])?
                        != std::cmp::Ordering::Equal
                    {
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        };
        match op {
            pyo3::basic::CompareOp::Eq => compare(other),
            pyo3::basic::CompareOp::Ne => match compare(other) {
                Ok(res) => Ok(!res),
                Err(err) => Err(err),
            },
            _ => Err(PyNotImplementedError::new_err(
                "Comparison not implemented",
            )),
        }
    }
}

#[pyproto]
impl PySequenceProtocol for BFSSuccessors {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.bfs_successors.len())
    }

    fn __getitem__(
        &'p self,
        idx: isize,
    ) -> PyResult<(PyObject, Vec<PyObject>)> {
        if idx >= self.bfs_successors.len().try_into().unwrap() {
            Err(PyIndexError::new_err(format!("Invalid index, {}", idx)))
        } else {
            Ok(self.bfs_successors[idx as usize].clone())
        }
    }
}

/// A custom class for the return of node indices
///
/// This class is a container class for the results of functions that
/// return a list of node indices. It implements the Python sequence
/// protocol. So you can treat the return as a read-only sequence/list
/// that is integer indexed. If you want to use it as an iterator you
/// can by wrapping it in an ``iter()`` that will yield the results in
/// order.
///
/// For example::
///
///     import retworkx
///
///     graph = retworkx.generators.directed_path_graph(5)
///     nodes = retworkx.node_indexes(0)
///     # Index based access
///     third_element = nodes[2]
///     # Use as iterator
///     nodes_iter = iter(node)
///     first_element = next(nodes_iter)
///     second_element = next(nodes_iter)
///
#[pyclass(module = "retworkx")]
pub struct NodeIndices {
    pub nodes: Vec<usize>,
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for NodeIndices {
    fn __richcmp__(
        &self,
        other: &'p PySequence,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<bool> {
        let compare = |other: &PySequence| -> PyResult<bool> {
            if other.len()? as usize != self.nodes.len() {
                return Ok(false);
            }
            for i in 0..self.nodes.len() {
                let other_raw = other.get_item(i.try_into().unwrap())?;
                let other_value: usize = other_raw.extract()?;
                if other_value != self.nodes[i] {
                    return Ok(false);
                }
            }
            Ok(true)
        };
        match op {
            pyo3::basic::CompareOp::Eq => compare(other),
            pyo3::basic::CompareOp::Ne => match compare(other) {
                Ok(res) => Ok(!res),
                Err(err) => Err(err),
            },
            _ => Err(PyNotImplementedError::new_err(
                "Comparison not implemented",
            )),
        }
    }
}

#[pyproto]
impl PySequenceProtocol for NodeIndices {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.nodes.len())
    }

    fn __getitem__(&'p self, idx: isize) -> PyResult<usize> {
        if idx >= self.nodes.len().try_into().unwrap() {
            Err(PyIndexError::new_err(format!("Invalid index, {}", idx)))
        } else {
            Ok(self.nodes[idx as usize])
        }
    }
}
