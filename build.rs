use std::env;
use std::path::PathBuf;

fn main() {
    metadeps::probe().unwrap();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("cds_wfcq_head_ptr_t")
        .whitelist_type("cds_wfcq_head")
        .whitelist_type("cds_wfcq_node")
        .whitelist_type("cds_wfcq_tail")
        .whitelist_type("rcu_head")
        .whitelist_type("rcu_flavor_struct")
        .whitelist_var("cds_wfcq_ret_CDS_WFCQ_RET_DEST_EMPTY")
        .whitelist_var("cds_wfcq_ret_CDS_WFCQ_RET_DEST_NON_EMPTY")
        .whitelist_var("cds_wfcq_ret_CDS_WFCQ_RET_SRC_EMPTY")
        .whitelist_var("cds_wfcq_ret_CDS_WFCQ_RET_WOULDBLOCK")
        .whitelist_var("cds_wfcq_state_CDS_WFCQ_STATE_LAST")
        .whitelist_var("URCU_CALL_RCU_PAUSE")
        .whitelist_var("URCU_CALL_RCU_PAUSED")
        .whitelist_var("URCU_CALL_RCU_RT")
        .whitelist_var("URCU_CALL_RCU_RUNNING")
        .whitelist_var("URCU_CALL_RCU_STOP")
        .whitelist_var("URCU_CALL_RCU_STOPPED")
        .whitelist_var("CONFIG_RCU_HAVE_CLOCK_GETTIME")
        .whitelist_var("CONFIG_RCU_HAVE_FENCE")
        .whitelist_var("CONFIG_RCU_HAVE_FUTEX")
        .whitelist_var("CONFIG_RCU_SMP")
        .whitelist_function("urcu_atfork")
        .whitelist_function("__cds_wfcq_dequeue_blocking")
        .whitelist_function("__cds_wfcq_dequeue_nonblocking")
        .whitelist_function("__cds_wfcq_dequeue_with_state_blocking")
        .whitelist_function("__cds_wfcq_dequeue_with_state_nonblocking")
        .whitelist_function("__cds_wfcq_first_blocking")
        .whitelist_function("__cds_wfcq_first_nonblocking")
        .whitelist_function("__cds_wfcq_init")
        .whitelist_function("__cds_wfcq_next_blocking")
        .whitelist_function("__cds_wfcq_next_nonblocking")
        .whitelist_function("__cds_wfcq_splice_blocking")
        .whitelist_function("__cds_wfcq_splice_nonblocking")
        .whitelist_function("__ctype_get_mb_cur_max")
        .whitelist_function("call_rcu_after_fork_child_memb")
        .whitelist_function("call_rcu_after_fork_parent_memb")
        .whitelist_function("call_rcu_before_fork_memb")
        .whitelist_function("call_rcu_data_free_memb")
        .whitelist_function("call_rcu_memb")
        .whitelist_function("cds_wfcq_dequeue_blocking")
        .whitelist_function("cds_wfcq_dequeue_lock")
        .whitelist_function("cds_wfcq_dequeue_unlock")
        .whitelist_function("cds_wfcq_dequeue_with_state_blocking")
        .whitelist_function("cds_wfcq_destroy")
        .whitelist_function("cds_wfcq_empty")
        .whitelist_function("cds_wfcq_enqueue")
        .whitelist_function("cds_wfcq_init")
        .whitelist_function("cds_wfcq_node_init")
        .whitelist_function("cds_wfcq_splice_blocking")
        .whitelist_function("create_all_cpu_call_rcu_data_memb")
        .whitelist_function("create_call_rcu_data_memb")
        .whitelist_function("defer_rcu_memb")
        .whitelist_function("free_all_cpu_call_rcu_data_memb")
        .whitelist_function("get_call_rcu_data_memb")
        .whitelist_function("get_call_rcu_thread_memb")
        .whitelist_function("get_cpu_call_rcu_data_memb")
        .whitelist_function("get_default_call_rcu_data_memb")
        .whitelist_function("get_thread_call_rcu_data_memb")
        .whitelist_function("rcu_barrier_memb")
        .whitelist_function("rcu_cmpxchg_pointer_sym")
        .whitelist_function("rcu_defer_barrier_memb")
        .whitelist_function("rcu_defer_barrier_thread_memb")
        .whitelist_function("rcu_defer_register_thread_memb")
        .whitelist_function("rcu_defer_unregister_thread_memb")
        .whitelist_function("rcu_dereference_sym")
        .whitelist_function("rcu_init_memb")
        .whitelist_function("rcu_read_lock_memb")
        .whitelist_function("rcu_read_ongoing_memb")
        .whitelist_function("rcu_read_unlock_memb")
        .whitelist_function("rcu_register_thread_memb")
        .whitelist_function("rcu_register_thread")
        .whitelist_function("rcu_set_pointer_sym")
        .whitelist_function("rcu_unregister_thread_memb")
        .whitelist_function("rcu_xchg_pointer_sym")
        .whitelist_function("set_cpu_call_rcu_data_memb")
        .whitelist_function("set_thread_call_rcu_data_memb")
        .whitelist_function("synchronize_rcu_memb")
        .whitelist_function("urcu_register_rculfhash_atfork_memb")
        .whitelist_function("urcu_unregister_rculfhash_atfork_memb")
        .whitelist_function("rcu_register_thread")
        // from scripts/urcu-api-list.sh
        .whitelist_function("caa_container_of")
        .whitelist_function("caa_likely")
        .whitelist_function("caa_max")
        .whitelist_function("caa_unlikely")
        .whitelist_function("call_rcu")
        .whitelist_function("call_rcu_after_fork_child")
        .whitelist_function("call_rcu_after_fork_parent")
        .whitelist_function("call_rcu_before_fork")
        .whitelist_function("call_rcu_data_free")
        .whitelist_function("cds_hlist_add_head")
        .whitelist_function("cds_hlist_add_head_rcu")
        .whitelist_function("cds_hlist_del")
        .whitelist_function("cds_hlist_del_rcu")
        .whitelist_function("cds_hlist_entry")
        .whitelist_function("cds_hlist_for_each_entry")
        .whitelist_function("cds_hlist_for_each_entry_rcu")
        .whitelist_function("cds_hlist_for_each_entry_safe")
        .whitelist_function("CDS_INIT_HLIST_HEAD")
        .whitelist_function("CDS_INIT_LIST_HEAD")
        .whitelist_function("cds_lfht_add")
        .whitelist_function("cds_lfht_add_replace")
        .whitelist_function("cds_lfht_add_unique")
        .whitelist_function("cds_lfht_count_nodes")
        .whitelist_function("cds_lfht_del")
        .whitelist_function("cds_lfht_destroy")
        .whitelist_function("cds_lfht_first")
        .whitelist_function("cds_lfht_for_each")
        .whitelist_function("cds_lfht_for_each_duplicate")
        .whitelist_function("cds_lfht_for_each_entry")
        .whitelist_function("cds_lfht_for_each_entry_duplicate")
        .whitelist_function("cds_lfht_is_node_deleted")
        .whitelist_function("cds_lfht_iter_get_node")
        .whitelist_function("cds_lfht_lookup")
        .whitelist_function("cds_lfht_new")
        .whitelist_function("cds_lfht_next")
        .whitelist_function("cds_lfht_next_duplicate")
        .whitelist_function("cds_lfht_replace")
        .whitelist_function("cds_lfht_resize")
        .whitelist_function("cds_lfq_dequeue_rcu")
        .whitelist_function("cds_lfq_destroy_rcu")
        .whitelist_function("cds_lfq_enqueue_rcu")
        .whitelist_function("cds_lfq_init_rcu")
        .whitelist_function("cds_lfq_node_init_rcu")
        .whitelist_function("cds_lfs_empty")
        .whitelist_function("cds_lfs_for_each")
        .whitelist_function("cds_lfs_for_each_safe")
        .whitelist_function("cds_lfs_init")
        .whitelist_function("cds_lfs_node_init")
        .whitelist_function("__cds_lfs_pop")
        .whitelist_function("__cds_lfs_pop_all")
        .whitelist_function("cds_lfs_pop_all_blocking")
        .whitelist_function("cds_lfs_pop_blocking")
        .whitelist_function("cds_lfs_pop_lock")
        .whitelist_function("cds_lfs_pop_unlock")
        .whitelist_function("cds_lfs_push")
        .whitelist_function("cds_list_add")
        .whitelist_function("cds_list_add_rcu")
        .whitelist_function("cds_list_add_tail")
        .whitelist_function("cds_list_del")
        .whitelist_function("cds_list_del_init")
        .whitelist_function("cds_list_del_rcu")
        .whitelist_function("cds_list_empty")
        .whitelist_function("cds_list_entry")
        .whitelist_function("cds_list_first_entry")
        .whitelist_function("cds_list_for_each")
        .whitelist_function("cds_list_for_each_entry")
        .whitelist_function("cds_list_for_each_entry_rcu")
        .whitelist_function("cds_list_for_each_entry_reverse")
        .whitelist_function("cds_list_for_each_prev")
        .whitelist_function("cds_list_for_each_prev_safe")
        .whitelist_function("cds_list_for_each_rcu")
        .whitelist_function("cds_list_for_each_safe")
        .whitelist_function("CDS_LIST_HEAD")
        .whitelist_function("CDS_LIST_HEAD_INIT")
        .whitelist_function("cds_list_move")
        .whitelist_function("cds_list_replace")
        .whitelist_function("cds_list_replace_init")
        .whitelist_function("cds_list_replace_rcu")
        .whitelist_function("cds_list_splice")
        .whitelist_function("__cds_wfcq_dequeue_blocking")
        .whitelist_function("cds_wfcq_dequeue_blocking")
        .whitelist_function("cds_wfcq_dequeue_lock")
        .whitelist_function("__cds_wfcq_dequeue_nonblocking")
        .whitelist_function("cds_wfcq_dequeue_unlock")
        .whitelist_function("cds_wfcq_empty")
        .whitelist_function("cds_wfcq_enqueue")
        .whitelist_function("__cds_wfcq_first_blocking")
        .whitelist_function("__cds_wfcq_first_nonblocking")
        .whitelist_function("__cds_wfcq_for_each_blocking")
        .whitelist_function("__cds_wfcq_for_each_blocking_safe")
        .whitelist_function("cds_wfcq_init")
        .whitelist_function("__cds_wfcq_next_blocking")
        .whitelist_function("__cds_wfcq_next_nonblocking")
        .whitelist_function("cds_wfcq_node_init")
        .whitelist_function("__cds_wfcq_splice_blocking")
        .whitelist_function("cds_wfcq_splice_blocking")
        .whitelist_function("__cds_wfcq_splice_nonblocking")
        .whitelist_function("cds_wfs_empty")
        .whitelist_function("cds_wfs_first")
        .whitelist_function("cds_wfs_for_each_blocking")
        .whitelist_function("cds_wfs_for_each_blocking_safe")
        .whitelist_function("cds_wfs_init")
        .whitelist_function("cds_wfs_next_blocking")
        .whitelist_function("cds_wfs_next_nonblocking")
        .whitelist_function("cds_wfs_node_init")
        .whitelist_function("__cds_wfs_pop_all")
        .whitelist_function("cds_wfs_pop_all_blocking")
        .whitelist_function("__cds_wfs_pop_blocking")
        .whitelist_function("cds_wfs_pop_blocking")
        .whitelist_function("cds_wfs_pop_lock")
        .whitelist_function("__cds_wfs_pop_nonblocking")
        .whitelist_function("cds_wfs_pop_unlock")
        .whitelist_function("cds_wfs_push")
        .whitelist_function("CMM_ACCESS_ONCE")
        .whitelist_function("cmm_barrier")
        .whitelist_function("CMM_LOAD_SHARED")
        .whitelist_function("cmm_smp_mb")
        .whitelist_function("cmm_smp_mb__after_uatomic_add")
        .whitelist_function("cmm_smp_mb__after_uatomic_and")
        .whitelist_function("cmm_smp_mb__after_uatomic_dec")
        .whitelist_function("cmm_smp_mb__after_uatomic_inc")
        .whitelist_function("cmm_smp_mb__after_uatomic_or")
        .whitelist_function("cmm_smp_mb__before_uatomic_add")
        .whitelist_function("cmm_smp_mb__before_uatomic_and")
        .whitelist_function("cmm_smp_mb__before_uatomic_dec")
        .whitelist_function("cmm_smp_mb__before_uatomic_inc")
        .whitelist_function("cmm_smp_mb__before_uatomic_or")
        .whitelist_function("cmm_smp_rmb")
        .whitelist_function("cmm_smp_wmb")
        .whitelist_function("CMM_STORE_SHARED")
        .whitelist_function("create_all_cpu_call_rcu_data")
        .whitelist_function("create_call_rcu_data")
        .whitelist_function("DECLARE_URCU_TLS")
        .whitelist_function("defer_rcu")
        .whitelist_function("DEFINE_URCU_TLS")
        .whitelist_function("free_all_cpu_call_rcu_data")
        .whitelist_function("get_call_rcu_data")
        .whitelist_function("get_call_rcu_thread")
        .whitelist_function("get_cpu_call_rcu_data")
        .whitelist_function("get_default_call_rcu_data")
        .whitelist_function("get_thread_call_rcu_data")
        .whitelist_function("rcu_assign_pointer")
        .whitelist_function("rcu_cmpxchg_pointer")
        .whitelist_function("rcu_dereference")
        .whitelist_function("rcu_exit")
        .whitelist_function("rcu_init")
        .whitelist_function("rcu_quiescent_state")
        .whitelist_function("rcu_read_lock")
        .whitelist_function("rcu_read_unlock")
        .whitelist_function("rcu_register_thread")
        .whitelist_function("rcu_set_pointer")
        .whitelist_function("rcu_thread_offline")
        .whitelist_function("rcu_thread_online")
        .whitelist_function("rcu_unregister_thread")
        .whitelist_function("rcu_xchg_pointer")
        .whitelist_function("set_cpu_call_rcu_data")
        .whitelist_function("set_thread_call_rcu_data")
        .whitelist_function("synchronize_rcu")
        .whitelist_function("uatomic_add")
        .whitelist_function("uatomic_add_return")
        .whitelist_function("uatomic_and")
        .whitelist_function("uatomic_cmpxchg")
        .whitelist_function("uatomic_dec")
        .whitelist_function("uatomic_inc")
        .whitelist_function("uatomic_or")
        .whitelist_function("uatomic_read")
        .whitelist_function("uatomic_set")
        .whitelist_function("uatomic_xchg")

        //.blacklist_item("__pthread_internal_list")
        //.blacklist_item("__pthread_mutex_s")
        //.blacklist_item("__pthread_list_t")
        //.blacklist_item("pthread_mutex_t")
        .derive_debug(true)
        .impl_debug(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}