use {
    crate::error::GlueSQLError,
    gluesql_json_storage::JsonStorage,
    gluesql_memory_storage::MemoryStorage,
    gluesql_shared_memory_storage::SharedMemoryStorage,
    gluesql_sled_storage::{SledStorage, sled},
    pyo3::prelude::*,
    std::path::PathBuf,
};

#[derive(FromPyObject)]
pub enum PyStorageEngine {
    Memory(PyMemoryStorage),
    Json(PyJsonStorage),
    SharedMemory(PySharedMemoryStorage),
    Sled(PySledStorage),
}

#[pyclass(name = "MemoryStorage", from_py_object)]
#[derive(Clone, Default)]
pub struct PyMemoryStorage(pub MemoryStorage);

#[pymethods]
impl PyMemoryStorage {
    #[new]
    pub fn new() -> Self {
        PyMemoryStorage::default()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(name = "JsonStorage", from_py_object)]
#[derive(Clone)]
pub struct PyJsonStorage(pub JsonStorage);

#[pymethods]
impl PyJsonStorage {
    #[new]
    pub fn new(path_arg: &str) -> Self {
        let mut path = PathBuf::new();
        path.push(path_arg);
        PyJsonStorage(JsonStorage { path })
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(name = "SharedMemoryStorage", from_py_object)]
#[derive(Clone, Default)]
pub struct PySharedMemoryStorage(pub SharedMemoryStorage);

#[pymethods]
impl PySharedMemoryStorage {
    #[new]
    pub fn new() -> Self {
        PySharedMemoryStorage::default()
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyclass(name = "SledStorageConfigMode", from_py_object)]
#[derive(Clone, Debug)]
pub struct PySledStorageModeConfig(pub sled::Mode);

#[pymethods]
impl PySledStorageModeConfig {
    pub fn __repr__(&self) -> String {
        match self.0 {
            sled::Mode::LowSpace => "LowSpace".to_owned(),
            sled::Mode::HighThroughput => "HighThroughput".to_owned(),
        }
    }
}

impl Default for PySledStorageModeConfig {
    fn default() -> Self {
        PySledStorageModeConfig(sled::Mode::LowSpace)
    }
}

#[pyclass(name = "SledStorageConfig", from_py_object)]
#[derive(Clone, Default, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct PySledStorageConfig {
    #[pyo3(get, set)]
    pub cache_capacity: u64,

    #[pyo3(get, set)]
    pub path: String,

    #[pyo3(get, set)]
    pub create_new: bool,

    #[pyo3(get, set)]
    pub mode: PySledStorageModeConfig,

    #[pyo3(get, set)]
    pub temporary: bool,

    #[pyo3(get, set)]
    pub use_compression: bool,

    #[pyo3(get, set)]
    pub compression_factor: i32,

    #[pyo3(get, set)]
    pub print_profile_on_drop: bool,
}

#[pymethods]
impl PySledStorageConfig {
    #[new]
    pub fn new() -> Self {
        PySledStorageConfig::default()
    }

    pub fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(name = "SledStorage", from_py_object)]
#[derive(Clone)]
pub struct PySledStorage(pub SledStorage);

#[pymethods]
impl PySledStorage {
    #[new]
    pub fn new(path_arg: &str) -> PyResult<Self> {
        let storage =
            SledStorage::new(path_arg).map_err(|e| GlueSQLError::new_err(e.to_string()))?;

        Ok(PySledStorage(storage))
    }

    #[staticmethod]
    pub fn try_from(cfg: &PySledStorageConfig) -> PyResult<Self> {
        let sled_cfg = sled::Config::default()
            .cache_capacity(cfg.cache_capacity)
            .compression_factor(cfg.compression_factor)
            .create_new(cfg.create_new)
            .mode(cfg.mode.0)
            .path(&cfg.path)
            .print_profile_on_drop(cfg.print_profile_on_drop)
            .temporary(cfg.temporary)
            .use_compression(cfg.use_compression);

        let storage =
            SledStorage::try_from(sled_cfg).map_err(|e| GlueSQLError::new_err(e.to_string()))?;

        Ok(PySledStorage(storage))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
