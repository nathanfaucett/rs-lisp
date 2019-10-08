(let package {
  :name "std-core"
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "std core"
  :deps {
    :core { :version "0.1" :path "./core" }
    :numbers { :version "0.1" :path "./numbers" }
  }
})

(export package)