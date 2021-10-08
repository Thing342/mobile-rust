package org.wesj.mobilerust

object JNI {
    interface JNICallback {
        fun callback(string: String)
    }

    init {
        System.loadLibrary("mobilerust")
        setupLogging()
    }

    private external fun setupLogging()

    private external fun invokeCallbackViaJNI(callback: JNICallback)
    fun invokeCallback(callback: (String)->Unit) {
        invokeCallbackViaJNI(object : JNICallback {
            override fun callback(string: String) {
                callback(string)
            }
        })
    }

    private external fun getATCFInfoJNI(request: CycloneMessageRequest): Any
    fun getATCFInfo(request: CycloneMessageRequest): CycloneMessage = getATCFInfoJNI(request) as CycloneMessage

}