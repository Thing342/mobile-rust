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

    private external fun getATCFInfoJNI(atcfId: String): String
    fun getATCFInfo(atcfId: String): String =  getATCFInfoJNI(atcfId)

}