(let package {
  :name :std
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "lisp standard lib"
  :deps {
    :std-core { :version "0.1" :path "./core" }
    :std-numbers { :version "0.1" :path "./numbers" }
    :std-fs { :version "0.1" :path "./fs" }
  }
})

(export package)