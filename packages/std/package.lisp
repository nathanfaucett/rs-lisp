(let package {
  :name :std
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "lisp standard lib"
  :deps {
    :std-core { :version "0.1" :path "./core" }
    :std-num { :version "0.1" :path "./num" }
    :std-fs { :version "0.1" :path "./fs" }
  }
})

(export package)