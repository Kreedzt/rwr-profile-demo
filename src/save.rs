use crate::model::Person;
use quick_xml::events::{BytesEnd, BytesStart, Event, BytesText};
use quick_xml::Writer;
use std::io::Cursor;

pub fn save_person(p: &Person) -> Result<String, ()> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

    let mut person_tag = BytesStart::owned(b"person".to_owned(), "person".len());

    person_tag.push_attribute((
        "max_authority_reached",
        p.max_authority_reached.to_string().as_str(),
    ));
    person_tag.push_attribute(("authority", p.authority.to_string().as_str()));
    person_tag.push_attribute(("job_points", p.job_points.to_string().as_str()));
    person_tag.push_attribute(("faction", p.faction.to_string().as_str()));
    person_tag.push_attribute(("name", p.name.as_str()));
    person_tag.push_attribute(("version", p.version.as_str()));
    person_tag.push_attribute(("alive", p.alive.to_string().as_str()));
    person_tag.push_attribute(("soldier_group_id", p.soldier_group_id.to_string().as_str()));
    person_tag.push_attribute(("soldier_group_name", p.soldier_group_name.as_str()));
    person_tag.push_attribute(("block", p.block.as_str()));
    person_tag.push_attribute((
        "squad_size_setting",
        p.squad_size_setting.to_string().as_str(),
    ));

    writer.write_event(Event::Start(person_tag));

    let mut order_tag = BytesStart::owned(b"order".to_owned(), "order".len());

    order_tag.push_attribute(("moving", p.order.moving.to_string().as_str()));
    order_tag.push_attribute(("target", p.order.target.to_string().as_str()));
    order_tag.push_attribute(("class", p.order.class.to_string().as_str()));


    writer.write_event(Event::Empty(order_tag));

    for item in p.item_list.iter() {
        let mut item_tag = BytesStart::owned(b"item".to_owned(), "item".len());

        item_tag.push_attribute(("slot", item.slot.to_string().as_str()));
        item_tag.push_attribute(("index", item.index.to_string().as_str()));
        item_tag.push_attribute(("amount", item.amount.to_string().as_str()));
        item_tag.push_attribute(("key", item.key.as_str()));

        writer.write_event(Event::Empty(item_tag));
    }

    let mut stash_tag = BytesStart::owned(b"stash".to_owned(), "stash".len());

    writer.write_event(Event::Start(stash_tag));

    for item in p.stash_item_list.iter() {
        let mut stash_item_tag = BytesStart::owned(b"item".to_owned(), "item".len());

        stash_item_tag.push_attribute(("class", item.class.to_string().as_str()));
        stash_item_tag.push_attribute(("index", item.index.to_string().as_str()));
        stash_item_tag.push_attribute(("key", item.key.as_str()));

        writer.write_event(Event::Empty(stash_item_tag));
    }

    writer.write_event(Event::End(BytesEnd::borrowed(b"stash")));

    let mut backpack_tag = BytesStart::owned(b"backpack".to_owned(), "backpack".len());

    writer.write_event(Event::Empty(backpack_tag));

    writer.write_event(Event::End(BytesEnd::borrowed(b"person")));

    let result = String::from_utf8(writer.into_inner().into_inner()).unwrap();

    Ok(result)
}
