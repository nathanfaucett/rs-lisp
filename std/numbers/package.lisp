(let package {
  :name :std-numbers
  :version "0.1.0"
  :authors ["Nathan Faucett nathanfaucett@gmail.com"]
  :description "std numbers"
  :deps {
    :std-core { :version "0.1", :path "../core" }
  }
})

(export package)