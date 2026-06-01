use {
    gluesql_core::prelude::Error,
    pyo3::{
        IntoPyObjectExt, create_exception, exceptions::PyException, prelude::*, pyclass::CompareOp,
    },
};

#[pyclass(name = "GlueSQLError")]
pub struct PyGlueSQLError(pub Error);

create_exception!(gluesql, GlueSQLError, PyException);

#[pymethods]
impl PyGlueSQLError {
    pub fn __richcmp__(
        &self,
        py: Python<'_>,
        rhs: &PyGlueSQLError,
        op: CompareOp,
    ) -> PyResult<Py<PyAny>> {
        match op {
            CompareOp::Eq => (self.0 == rhs.0).into_py_any(py),
            CompareOp::Ne => (self.0 != rhs.0).into_py_any(py),
            _ => Ok(py.NotImplemented()),
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}

impl From<PyGlueSQLError> for PyErr {
    fn from(e: PyGlueSQLError) -> PyErr {
        GlueSQLError::new_err(e.0.to_string())
    }
}
