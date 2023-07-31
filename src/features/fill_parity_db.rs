
fn fill_parity_db() {
    let db_path = "/Users/shamix/fill_parity_db_test";
    let cache_size = NonZeroUsize::new(100000).unwrap();
    let local_peer_id = PeerId::from_multihash(Multihash::wrap(0, &vec![0,1]).unwrap()).unwrap();
    let mut storage_provider = ParityDbProviderStorage::new(&Path::new(db_path), cache_size, local_peer_id).unwrap();
    let provider = PeerId::from_multihash(Multihash::wrap(0, &vec![2,3, 4]).unwrap()).unwrap();

    return;

    let objects_number = 100000u64;
    for i in 0..objects_number {
        //   let key: Multihash = Multihash::from_bytes(&i.to_le_bytes().to_vec()).unwrap();
        let key: Key = i.to_le_bytes().to_vec().into();
        // storage_provider.add_provider(ProviderRecord{
        //     key: key.clone(),
        //     provider,
        //     expires: None,
        //     addresses: vec![],
        // }).unwrap();

        storage_provider.remove_provider(&key, &provider);

        if i % 10000 == 0 {
            info!("{} objects added", i);
        }
    }
}
