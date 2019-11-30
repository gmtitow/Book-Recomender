#[derive(Default)]
pub struct CustomQuery{
    columns: Option<String>,
    from: Option<String>,
    condition: Option<String>,
    //bind: Option<String>,
    order: Option<String>,  
    group: Option<String>,
    limit: Option<String>, 
    offset: Option<String>, 
    distinct: Option<String>,
    option: Option<String>, 
    // bind: Option<Vec<ToSql>>
}

impl CustomQuery{
    pub fn new() -> CustomQuery{
        CustomQuery {
            ..CustomQuery::default()
        }
    }

    pub fn set_columns(& mut self, columns : &str) -> &Self {
        self.columns = Some(columns.to_owned());

        self
    }

    pub fn get_columns(&self)->&Option<String>{
        &self.columns
    }

    pub fn set_from(&mut self, from : &str)->&Self{
        self.from = Some(from.to_owned());

        self
    }

    pub fn get_from(&self)->&Option<String>{
        &self.from
    }

    pub fn set_condition(&mut self, condition : &str)->&Self{
        self.condition = Some(condition.to_owned());

        self
    }

    pub fn get_condition(&self)->&Option<String>{
        &self.condition
    }

    pub fn set_order(&mut self, order : &str)->&Self{
        self.order = Some(order.to_owned());

        self
    }

    pub fn get_order(&self)->&Option<String>{
        &self.order
    }

    pub fn set_group(&mut self, group : &str)->&Self{
        self.group = Some(group.to_owned());

        self
    }

    pub fn get_group(&self)->&Option<String>{
        &self.group
    }

    pub fn set_limit(&mut self, limit : &str)->&Self{
        self.limit = Some(limit.to_owned());

        self
    }

    pub fn get_limit(&self)->&Option<String>{
        &self.limit
    }

    pub fn set_offset(&mut self, offset : &str)->&Self{
        self.offset = Some(offset.to_owned());

        self
    }

    pub fn get_offset(&self)->&Option<String>{
        &self.offset
    }

    pub fn set_distinct(&mut self, distinct : &str)->&Self{
        self.distinct = Some(distinct.to_owned());

        self
    }

    pub fn get_distinct(&self)->&Option<String>{
        &self.distinct
    }

    pub fn set_option(&mut self, option : &str)->&Self{
        self.option = Some(option.to_owned());

        self
    }

    pub fn get_option(&self)->&Option<String>{
        &self.option
    }

    // pub fn set_bind(&mut self, bind : Vec<&ToSql>)->&Self{
    //     self.bind = Some(bind);

    //     self
    // }

    // pub fn get_bind(&self)->&Option<Vec<&ToSql>>{
    //     &self.bind
    // }
}