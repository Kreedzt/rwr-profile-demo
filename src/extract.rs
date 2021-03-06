use crate::model::{ItemTag, OrderTag, Person, StashItemTag};
use quick_xml::{events::Event, Reader};
use std::str;

pub fn extract_person() -> Result<Person, String> {
    let mut person = Person::default();

    let mut reader = Reader::from_file("1432226718.person").unwrap();

    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut is_in_person = false;
    let mut is_in_stash = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                // println!("start e.name: {:?}", str::from_utf8(e.name()).unwrap());
                match e.name() {
                    b"person" => {
                        is_in_person = true;

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr.unwrap();
                            let attr_value =
                                attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                            let attr_key = attr_unwrap_res.key;

                            match attr_key {
                                b"max_authority_reached" => {
                                    person.max_authority_reached = attr_value.parse().unwrap();
                                }
                                b"authority" => {
                                    person.authority = attr_value.parse().unwrap();
                                }
                                b"job_points" => {
                                    person.job_points = attr_value.parse().unwrap();
                                }
                                b"faction" => {
                                    person.faction = attr_value;
                                }
                                b"name" => {
                                    person.name = attr_value;
                                }
                                b"version" => {
                                    person.version = attr_value;
                                }
                                b"alive" => {
                                    person.alive = attr_value.parse().unwrap();
                                }
                                b"soldier_group_id" => {
                                    person.soldier_group_id = attr_value.parse().unwrap();
                                }
                                b"soldier_group_name" => {
                                    person.soldier_group_name = attr_value;
                                }
                                b"block" => {
                                    person.block = attr_value;
                                }
                                b"squad_size_setting" => {
                                    person.squad_size_setting = attr_value.parse().unwrap();
                                }
                                b"order" => {
                                    let mut order_item = OrderTag::default();
                                }
                                _ => (),
                            }
                        }
                        println!("This is person");
                    }
                    b"stash" => {
                        is_in_stash = true;
                        println!("This is stash");
                    }
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                println!("empty e.name: {:?}", str::from_utf8(e.name()).unwrap());

                match e.name() {
                    b"order" => {
                        let mut order_item = OrderTag::default();

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr.unwrap();
                            let attr_value =
                                attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                            let attr_key = attr_unwrap_res.key;

                            println!(
                                "attr: {}, value: {}",
                                str::from_utf8(attr_key).unwrap(),
                                attr_value
                            );

                            match attr_key {
                                b"moving" => {
                                    order_item.moving = attr_value.parse().unwrap();
                                }
                                b"target" => {
                                    order_item.target = attr_value;
                                }
                                b"class" => {
                                    order_item.class = attr_value.parse().unwrap();
                                }
                                _ => (),
                            }
                        }

                        person.order = order_item;
                    }
                    b"item" => {
                        if is_in_stash {
                            let mut stash_item = StashItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                println!(
                                    "attr: {}, value: {}",
                                    str::from_utf8(attr_key).unwrap(),
                                    attr_value
                                );

                                match attr_key {
                                    b"class" => {
                                        stash_item.class = attr_value.parse().unwrap();
                                    }
                                    b"index" => {
                                        stash_item.index = attr_value.parse().unwrap();
                                    }
                                    b"key" => {
                                        stash_item.key = attr_value.parse().unwrap();
                                    }
                                    _ => (),
                                }
                            }

                            person.stash_item_list.push(stash_item);
                        } else {
                            let mut item_tag = ItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr.unwrap();
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader).unwrap();
                                let attr_key = attr_unwrap_res.key;

                                println!(
                                    "attr: {}, value: {}",
                                    str::from_utf8(attr_key).unwrap(),
                                    attr_value
                                );

                                match attr_key {
                                    b"slot" => {
                                        item_tag.slot = attr_value.parse().unwrap();
                                    }
                                    b"index" => {
                                        item_tag.index = attr_value.parse().unwrap();
                                    }
                                    b"amount" => {
                                        item_tag.amount = attr_value.parse().unwrap();
                                    }
                                    b"key" => {
                                        item_tag.key = attr_value;
                                    }
                                    _ => (),
                                }
                            }

                            person.item_list.push(item_tag);
                        }
                    }
                    b"backpack" => {}
                    _ => (),
                }
            }
            Ok(Event::End(e)) => {
                println!("end e.name: {:?}", str::from_utf8(e.name()).unwrap());
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {} : {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    Ok(person)
}
