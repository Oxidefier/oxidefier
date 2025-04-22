module List = struct
  include List
  let rmap l f = map f l
end
