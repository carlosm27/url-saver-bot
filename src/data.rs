use once_cell::sync::Lazy;

use std::sync::Mutex;

static DATA: Lazy<Mutex<HashMap<UUID, StoredURL>>> = Lazy::new(|| Mutex::new(
    HashMap::from([
        
    ])
));