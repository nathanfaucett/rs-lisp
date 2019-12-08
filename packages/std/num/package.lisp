(let package {
  :name :std-num
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "std number helpers"
  :deps {
    :std-core { :version "0.1", :path "../core" }
  }
})

(export package)