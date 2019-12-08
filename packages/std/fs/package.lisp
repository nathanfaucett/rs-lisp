(let package {
  :name :std-fs
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "std file system"
  :deps {
    :std-core { :version "0.1", :path "../core" }
  }
})

(export package)