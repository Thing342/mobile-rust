package org.wesj.mobilerust

object JNI {
    interface JNICallback {
        fun callback(string: String)
    }

    init {
        System.loadLibrary("mobilerust")
    }

    private external fun invokeCallbackViaJNI(callback: JNICallback)

    fun invokeCallback(callback: (String)->Unit) {
        invokeCallbackViaJNI(object : JNICallback {
            override fun callback(string: String) {
                callback(string)
            }
        })
    }
}