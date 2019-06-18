// Reference pointers point to a resource in memory

pub fn run()
{
    // Primitive - Array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // Non-primitive - Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));
}