use jni::objects::JClass;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Main_nativeStaticFn (
	env: JNIEnv,
	_class: JClass,
) {
	let cls = env
		.find_class ( "Main" )
		.expect ( "Failed to load thetarget class" );
	let result = env.call_static_method ( cls, "showIt", "()V", &[] );

	result.map_err ( |e| e.to_string ( ) ).unwrap ( );
}

