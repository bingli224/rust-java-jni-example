
/**
 * Test Java+Rust with JNI
 *
 * 12:26 THA 02/05/2020
 *
 * References:
 *	https://docs.rs/jni/0.16.0/jni/struct.JavaVM.html
 *	https://stackoverflow.com/questions/56821728/how-can-i-invoke-a-java-method-from-rust-via-jni
 */
use jni::{
	self,
	JavaVM,
	JNIVersion,
	InitArgsBuilder,
	objects::{
		JValue,
	},
	sys::{
		jint,
	},
};

fn main() {
	let jvm_args = InitArgsBuilder::new ( )
		.version ( JNIVersion::V8 )
		.option ( "-Xcheck:jni" )
		.build ( )
		.unwrap ( );

	let jvm = JavaVM::new ( jvm_args )
		.unwrap ( );
	let env = jvm.attach_current_thread ( )
		.unwrap ( );

	// test Math.abs()

	let x = JValue::from ( -10 );
	let val: jint = env.call_static_method (
			"java/lang/Math",
			"abs",
			"(I)I",
			&[ x ]
		)
		.unwrap ( )
		.i ( )
		.unwrap ( );

	assert_eq ! ( val, 10 );

	// test System.out.println()

	let system = env.find_class ( "java/lang/System" )
		.unwrap ( );
	let _print_stream = env.find_class ( "java/io/PrintStream" )
		.unwrap ( );

	let out = env
		.get_static_field ( system, "out", "Ljava/io/PrintStream;" )
		.unwrap ( );

	if let JValue::Object ( out ) = out {
		let message = env.new_string ( "System.out.println() from rust" )
			.unwrap ( );

		env.call_method (
			out,
			"println",	// method name
			"(Ljava/lang/String;)V", // method signature
			&[JValue::Object ( message.into ( ) ) ],
		).unwrap ( );
	}

	// test my Main.showIt()

	//Java_Main_nativeStaticFn ( env );
	
	let my_class = env.find_class ( "Main" )
		.unwrap ( );
	
	env.call_static_method (
		//"Main",
		//env.find_class ( "Main" ).unwrap ( ),
		my_class,
		"showIt",
		"()V",
		& [ ]
	)
		.unwrap ( );
}
