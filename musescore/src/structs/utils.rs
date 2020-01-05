pub fn remove_element<T: PartialEq>(vec: &mut Vec<T>, e: &T) {
	if let Some(i) = vec.iter().position(|o| o == e) {
		vec.remove(i);
	}
}