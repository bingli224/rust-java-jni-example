
public class Main {
	static {
		// absolute path
		//System.load ( "D:\\Workspace\\Rust\\test_jni\\target\\debug\\test_jni.dll" );

		// library in current path without file extension
		System.loadLibrary ( "test_jni" );
	}

	public static void showIt ( ) {
		System.out.println ( "this path: \u001b[32m" + new java.io.File ( "" ).getAbsolutePath ( ) + "\u001b[37m" );
		System.out.println ( "test \u001b[33mSystem.out.println()\u001b[37m" );
	}

	public static native void nativeStaticFn ( );

	public static void main ( String [] argv ) {
		showIt ( );
	}
}
