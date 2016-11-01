// Composition. Challenging to maintain static dispatch
// Events/State. Channels with many tx downtree into one rx at top of tree. Should facilitate arbitrary state abstractions, fits well with rust ownership.
// Dynamic children. Heap allocation necessary, dynamic dispatch on heterogenous children. Virtualization over bounded, static children might help here, but inadquate for large/recursive trees
// Keys? Reconciling now is effecitvely a zip operation. Won't perform well unless list is stable. Lists are usually stable, using the approach I came up with at Netflix.

#![feature(conservative_impl_trait)]
#![feature(core_intrinsics)]

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum HTMLTag {
    a = 0,
    abbr = 1,
    address = 2,
    area = 3,
    article = 4,
    aside = 5,
    audio = 6,
    b = 7,
    base = 8,
    bdi = 9,
    bdo = 10,
    blockquote = 11,
    body = 12,
    br = 13,
    button = 14,
    canvas = 15,
    caption = 16,
    cite = 17,
    code = 18,
    col = 19,
    colgroup = 20,
    data = 21,
    datalist = 22,
    dd = 23,
    del = 24,
    details = 25,
    dfn = 26,
    div = 27,
    dl = 28,
    dt = 29,
    element = 30,
    em = 31,
    embed = 32,
    fieldset = 33,
    figcaption = 34,
    figure = 35,
    footer = 36,
    form = 37,
    head = 38,
    header = 39,
    hr = 40,
    html = 41,
    i = 42,
    iframe = 43,
    img = 44,
    input = 45,
    ins = 46,
    kbd = 47,
    label = 48,
    legend = 49,
    li = 50,
    link = 51,
    main = 52,
    map = 53,
    mark = 54,
    meta = 55,
    meter = 56,
    nav = 57,
    noframes = 58,
    noscript = 59,
    object = 60,
    ol = 61,
    optgroup = 62,
    option = 63,
    output = 64,
    p = 65,
    param = 66,
    pre = 67,
    progress = 68,
    q = 69,
    rp = 70,
    rt = 71,
    rtc = 72,
    ruby = 73,
    s = 74,
    samp = 75,
    script = 76,
    section = 77,
    select = 78,
    shadow = 79,
    small = 80,
    source = 81,
    span = 82,
    strong = 83,
    style = 84,
    sub = 85,
    summary = 86,
    sup = 87,
    table = 88,
    tbody = 89,
    td = 90,
    template = 91,
    textarea = 92,
    tfoot = 93,
    th = 94,
    thead = 95,
    time = 96,
    title = 97,
    tr = 98,
    track = 99,
    u = 100,
    ul = 101,
    var = 102,
    video = 103,
    wbr = 104,
    EMPTY = 105
}

#[repr(C)]
pub enum HTMLAttribute {
    accept = 0,
    acceptCharset = 1,
    accesskey = 2,
    action = 3,
    align = 4,
    alt = 5,
    async = 6,
    autocomplete = 7,
    autofocus = 8,
    autoplay = 9,
    autosave = 10,
    bgcolor = 11,
    border = 12,
    buffered = 13,
    challenge = 14,
    charset = 15,
    checked = 16,
    cite = 17,
    class = 18,
    code = 19,
    codebase = 20,
    color = 21,
    cols = 22,
    colspan = 23,
    content = 24,
    contenteditable = 25,
    contextmenu = 26,
    controls = 27,
    coords = 28,
    data = 29,
    dataDash = 30,
    datetime = 31,
    default = 32,
    defer = 33,
    dir = 34,
    dirname = 35,
    disabled = 36,
    download = 37,
    draggable = 38,
    dropzone = 39,
    enctype = 40,
    _for = 41,
    form = 42,
    formaction = 43,
    headers = 44,
    height = 45,
    hidden = 46,
    high = 47,
    href = 48,
    hreflang = 49,
    httpEquiv = 50,
    icon = 51,
    id = 52,
    ismap = 53,
    itemprop = 54,
    keytype = 55,
    kind = 56,
    label = 57,
    lang = 58,
    language = 59,
    list = 60,
    _loop = 61,
    low = 62,
    manifest = 63,
    max = 64,
    maxlength = 65,
    media = 66,
    method = 67,
    min = 68,
    multiple = 69,
    muted = 70,
    name = 71,
    novalidate = 72,
    open = 73,
    optimum = 74,
    pattern = 75,
    ping = 76,
    placeholder = 77,
    poster = 78,
    preload = 79,
    radiogroup = 80,
    readonly = 81,
    rel = 82,
    required = 83,
    reversed = 84,
    rows = 85,
    rowspan = 86,
    sandbox = 87,
    scope = 88,
    scoped = 89,
    seamless = 90,
    selected = 91,
    shape = 92,
    size = 93,
    sizes = 94,
    span = 95,
    spellcheck = 96,
    src = 97,
    srcdoc = 98,
    srclang = 99,
    srcset = 100,
    start = 101,
    step = 102,
    style = 103,
    summary = 104,
    tabindex = 105,
    target = 106,
    title = 107,
    _type = 108,
    usemap = 109,
    value = 110,
    width = 111,
    wrap = 112
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSStyle {
    alignContent = 0,
    alignItems = 1,
    alignSelf = 2,
    alignmentBaseline = 3,
    all = 4,
    animation = 5,
    animationDelay = 6,
    animationDirection = 7,
    animationDuration = 8,
    animationFillMode = 9,
    animationIterationCount = 10,
    animationName = 11,
    animationPlayState = 12,
    animationTimingFunction = 13,
    backfaceVisibility = 14,
    background = 15,
    backgroundAttachment = 16,
    backgroundBlendMode = 17,
    backgroundClip = 18,
    backgroundColor = 19,
    backgroundImage = 20,
    backgroundOrigin = 21,
    backgroundPosition = 22,
    backgroundPositionX = 23,
    backgroundPositionY = 24,
    backgroundRepeat = 25,
    backgroundRepeatX = 26,
    backgroundRepeatY = 27,
    backgroundSize = 28,
    baselineShift = 29,
    border = 30,
    borderBottom = 31,
    borderBottomColor = 32,
    borderBottomLeftRadius = 33,
    borderBottomRightRadius = 34,
    borderBottomStyle = 35,
    borderBottomWidth = 36,
    borderCollapse = 37,
    borderColor = 38,
    borderImage = 39,
    borderImageOutset = 40,
    borderImageRepeat = 41,
    borderImageSlice = 42,
    borderImageSource = 43,
    borderImageWidth = 44,
    borderLeft = 45,
    borderLeftColor = 46,
    borderLeftStyle = 47,
    borderLeftWidth = 48,
    borderRadius = 49,
    borderRight = 50,
    borderRightColor = 51,
    borderRightStyle = 52,
    borderRightWidth = 53,
    borderSpacing = 54,
    borderStyle = 55,
    borderTop = 56,
    borderTopColor = 57,
    borderTopLeftRadius = 58,
    borderTopRightRadius = 59,
    borderTopStyle = 60,
    borderTopWidth = 61,
    borderWidth = 62,
    bottom = 63,
    boxShadow = 64,
    boxSizing = 65,
    breakAfter = 66,
    breakBefore = 67,
    breakInside = 68,
    bufferedRendering = 69,
    captionSide = 70,
    clear = 71,
    clip = 72,
    clipPath = 73,
    clipRule = 74,
    color = 75,
    colorInterpolation = 76,
    colorInterpolationFilters = 77,
    colorRendering = 78,
    columnCount = 79,
    columnFill = 80,
    columnGap = 81,
    columnRule = 82,
    columnRuleColor = 83,
    columnRuleStyle = 84,
    columnRuleWidth = 85,
    columnSpan = 86,
    columnWidth = 87,
    columns = 88,
    contain = 89,
    content = 90,
    counterIncrement = 91,
    counterReset = 92,
    cursor = 93,
    cx = 94,
    cy = 95,
    d = 96,
    direction = 97,
    display = 98,
    dominantBaseline = 99,
    emptyCells = 100,
    fill = 101,
    fillOpacity = 102,
    fillRule = 103,
    filter = 104,
    flex = 105,
    flexBasis = 106,
    flexDirection = 107,
    flexFlow = 108,
    flexGrow = 109,
    flexShrink = 110,
    flexWrap = 111,
    float = 112,
    floodColor = 113,
    floodOpacity = 114,
    font = 115,
    fontFamily = 116,
    fontFeatureSettings = 117,
    fontKerning = 118,
    fontSize = 119,
    fontStretch = 120,
    fontStyle = 121,
    fontVariant = 122,
    fontVariantCaps = 123,
    fontVariantLigatures = 124,
    fontVariantNumeric = 125,
    fontWeight = 126,
    height = 127,
    imageRendering = 128,
    isolation = 129,
    justifyContent = 130,
    left = 131,
    letterSpacing = 132,
    lightingColor = 133,
    lineHeight = 134,
    listStyle = 135,
    listStyleImage = 136,
    listStylePosition = 137,
    listStyleType = 138,
    margin = 139,
    marginBottom = 140,
    marginLeft = 141,
    marginRight = 142,
    marginTop = 143,
    marker = 144,
    markerEnd = 145,
    markerMid = 146,
    markerStart = 147,
    mask = 148,
    maskType = 149,
    maxHeight = 150,
    maxWidth = 151,
    maxZoom = 152,
    minHeight = 153,
    minWidth = 154,
    minZoom = 155,
    mixBlendMode = 156,
    motion = 157,
    motionOffset = 158,
    motionPath = 159,
    motionRotation = 160,
    objectFit = 161,
    objectPosition = 162,
    opacity = 163,
    order = 164,
    orientation = 165,
    orphans = 166,
    outline = 167,
    outlineColor = 168,
    outlineOffset = 169,
    outlineStyle = 170,
    outlineWidth = 171,
    overflow = 172,
    overflowWrap = 173,
    overflowX = 174,
    overflowY = 175,
    padding = 176,
    paddingBottom = 177,
    paddingLeft = 178,
    paddingRight = 179,
    paddingTop = 180,
    page = 181,
    pageBreakAfter = 182,
    pageBreakBefore = 183,
    pageBreakInside = 184,
    paintOrder = 185,
    perspective = 186,
    perspectiveOrigin = 187,
    pointerEvents = 188,
    position = 189,
    quotes = 190,
    r = 191,
    resize = 192,
    right = 193,
    rx = 194,
    ry = 195,
    shapeImageThreshold = 196,
    shapeMargin = 197,
    shapeOutside = 198,
    shapeRendering = 199,
    size = 200,
    speak = 201,
    src = 202,
    stopColor = 203,
    stopOpacity = 204,
    stroke = 205,
    strokeDasharray = 206,
    strokeDashoffset = 207,
    strokeLinecap = 208,
    strokeLinejoin = 209,
    strokeMiterlimit = 210,
    strokeOpacity = 211,
    strokeWidth = 212,
    tabSize = 213,
    tableLayout = 214,
    textAlign = 215,
    textAlignLast = 216,
    textAnchor = 217,
    textCombineUpright = 218,
    textDecoration = 219,
    textIndent = 220,
    textOrientation = 221,
    textOverflow = 222,
    textRendering = 223,
    textShadow = 224,
    textSizeAdjust = 225,
    textTransform = 226,
    top = 227,
    touchAction = 228,
    transform = 229,
    transformOrigin = 230,
    transformStyle = 231,
    transition = 232,
    transitionDelay = 233,
    transitionDuration = 234,
    transitionProperty = 235,
    transitionTimingFunction = 236,
    unicodeBidi = 237,
    unicodeRange = 238,
    userZoom = 239,
    vectorEffect = 240,
    verticalAlign = 241,
    visibility = 242,
    whiteSpace = 243,
    widows = 244,
    width = 245,
    willChange = 246,
    wordBreak = 247,
    wordSpacing = 248,
    wordWrap = 249,
    writingMode = 250,
    x = 251,
    y = 252,
    zIndex = 253,
    zoom = 254,
    EMPTY = 255
}

#[repr(C)]
pub enum DOMEvent {
    onabort = 0,
    onblur = 1,
    oncancel = 2,
    oncanplay = 3,
    oncanplaythrough = 4,
    onchange = 5,
    onclick = 6,
    onclose = 7,
    oncontextmenu = 8,
    oncuechange = 9,
    ondblclick = 10,
    ondrag = 11,
    ondragend = 12,
    ondragenter = 13,
    ondragleave = 14,
    ondragover = 15,
    ondragstart = 16,
    ondrop = 17,
    ondurationchange = 18,
    onemptied = 19,
    onended = 20,
    onerror = 21,
    onfocus = 22,
    oninput = 23,
    oninvalid = 24,
    onkeydown = 25,
    onkeypress = 26,
    onkeyup = 27,
    onload = 28,
    onloadeddata = 29,
    onloadedmetadata = 30,
    onloadstart = 31,
    onmousedown = 32,
    onmouseenter = 33,
    onmouseleave = 34,
    onmousemove = 35,
    onmouseout = 36,
    onmouseover = 37,
    onmouseup = 38,
    onmousewheel = 39,
    onpause = 40,
    onplay = 41,
    onplaying = 42,
    onprogress = 43,
    onratechange = 44,
    onreset = 45,
    onresize = 46,
    onscroll = 47,
    onseeked = 48,
    onseeking = 49,
    onselect = 50,
    onshow = 51,
    onstalled = 52,
    onsubmit = 53,
    onsuspend = 54,
    ontimeupdate = 55,
    ontoggle = 56,
    onvolumechange = 57,
    onwaiting = 58,
    onbeforecopy = 59,
    onbeforecut = 60,
    onbeforepaste = 61,
    oncopy = 62,
    oncut = 63,
    onpaste = 64,
    onsearch = 65,
    onselectstart = 66,
    onwheel = 67
}

#[repr(C)]
struct RustObject {
    a: i32,
    // other members
}

extern "C" fn rust_callback(target: *mut RustObject, a: i32) {
    unsafe {
        // Update the value in RustObject with the value received from the callback
        (*target).a = a + (*target).a;
        println!("Got event with {}! Target.a is now {}", a, (*target).a);
    }
}

#[link(name = "libvdom_compare")]
extern "C" {
    // fn register_callback(target: *mut RustObject,
    //                     cb: extern fn(*mut RustObject, i32)) -> i32;
    // fn trigger_callback();
    fn get_document() -> i32;
    fn get_document_body() -> i32;
    fn create_element(tag: HTMLTag) -> i32;
    fn destroy_element(element_id: i32) -> i32;
    fn set_element_parent(element_id: i32, parent_id: i32) -> i32;
    fn set_element_attribute_str(element_id: i32, attr: HTMLAttribute, value: *const c_char, length: i32) -> i32;
    fn set_element_attribute_f32(element_id: i32, attr: HTMLAttribute, value: f32) -> i32;
    fn set_element_style_str(element_id: i32, style: CSSStyle, value: *const c_char, length: i32) -> i32;
    fn set_element_style_f32(element_id: i32, style: CSSStyle, value: f32) -> i32;
    fn set_element_style_i32(element_id: i32, style: CSSStyle, value: i32) -> i32;
    fn reset_element_style(element_id: i32, style: CSSStyle) -> i32;
    fn add_event_listener(element_id: i32, event_type: DOMEvent, function_ptr: extern fn(*mut RustObject, i32), function_context: *mut RustObject) -> i32;
    fn remove_event_listener(element_id: i32, event_type: DOMEvent) -> i32;
    fn now() -> f32;
}

#[no_mangle]
pub fn callback_junk() {
    // // Create the object that will be referenced in the callback
    // let mut rust_object = Box::new(RustObject { a: 5 });

    // unsafe {
    //     register_callback(&mut *rust_object, rust_callback);
    //     trigger_callback();
    //     println!("I'm called from C with value {0}", rust_object.a);
    //     trigger_callback();
    //     println!("I'm called from C with value {0}", rust_object.a);
    // }
}

pub trait Styles where Self::R: Styles {
    type R;
    fn apply<'a>(&'a self, element_id: i32);
    fn reset<'a>(&'a self, element_id: i32);
    fn diff_apply<'a, P>(&'a self, prev_styles: &P, element_id: i32) where P: Styles;
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F);
    fn compare(&self, name: CSSStyle, value: &StyleValue) -> bool;
}

#[derive(Debug)]
pub struct EmptyStyles;
impl Styles for EmptyStyles {
    type R = EmptyStyles;

    #[inline]
    fn apply<'a>(&'a self, _: i32) {
    }

    #[inline]
    fn reset<'a>(&'a self, element_id: i32) {

    }

    #[inline]
    fn diff_apply<'a, P>(&'a self, prev_styles: &P, element_id: i32) where P: Styles {
        prev_styles.reset(element_id);
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&EmptyStyles);
    }

    #[inline]
    fn compare(&self, name: CSSStyle, value: &StyleValue) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StyleValue {
    Static(&'static str),
    Heap(String)
}

pub trait AsStyleValue {
    #[inline]
    fn as_style_value(self) -> StyleValue;
}

impl AsStyleValue for &'static str {
    #[inline]
    fn as_style_value(self) -> StyleValue {
        StyleValue::Static(self)
    }
}

impl AsStyleValue for String {
    #[inline]
    fn as_style_value(self) -> StyleValue {
        StyleValue::Heap(self)
    }
}

impl StyleValue {
    // TODO: Errors
    fn apply(&self, element_id: i32, style: CSSStyle) {
        let string = match *self {
            StyleValue::Static(string) => string,
            StyleValue::Heap(ref string) => string.as_str(),
        };

        unsafe {
            set_element_style_str(element_id, style, CString::new(string).unwrap().as_ptr(), string.len() as i32);
        }
    }
}

#[derive(Debug)]
pub struct StyleNode<R> where R: Styles {
    name: CSSStyle,
    value: StyleValue,
    sibling: R
}

impl StyleNode<EmptyStyles> {
    #[inline]
    pub fn new<V>(name: CSSStyle, value: V) -> StyleNode<EmptyStyles>
        where V: AsStyleValue
    {
        StyleNode {
            name: name,
            value: value.as_style_value(),
            sibling: EmptyStyles
        }
    }
}

impl<'n, R> StyleNode<R> where R: Styles {
    #[inline]
    pub fn add_sibling<V>(self, name: CSSStyle, value: V) -> StyleNode<StyleNode<R>>
        where V: AsStyleValue
    {
        StyleNode {
            name: name,
            value: value.as_style_value(),
            sibling: self
        }
    }
}

impl<'n, R> Styles for StyleNode<R> where R: Styles {
    type R = R;
    #[inline]
    fn apply<'a>(&'a self, element_id: i32) {
        self.sibling.apply(element_id);
        self.value.apply(element_id, self.name);
    }

    #[inline]
    fn reset<'a>(&'a self, element_id: i32) {
        unsafe {
            reset_element_style(element_id, self.name);
        }
    }

    #[inline]
    fn diff_apply<'a, P>(&'a self, prev_styles: &P, element_id: i32) where P: Styles {
        prev_styles.apply_sibling(|sibling| self.sibling.diff_apply(sibling, element_id));

        if !prev_styles.compare(self.name, &self.value) {
            self.value.apply(element_id, self.name);
        }
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&self.sibling);
    }

    #[inline]
    fn compare(&self, name: CSSStyle, value: &StyleValue) -> bool {
        self.name == name && self.value == *value
    }
}

#[macro_export]
macro_rules! style {
    ($field:ident: $value:expr) => {
        StyleNode::new(CSSStyle::$field, $value)
    };

    ($field1:ident: $value1:expr, $($field:ident: $value:expr),+) => {
        {
            let styles = StyleNode::new(CSSStyle::$field1, $value1);
            $(
                let styles = styles.add_sibling(CSSStyle::$field, $value);
            )+
            styles
        }
    };

    ($($field:ident: $value:expr),*,) => {
        style!($($field: $value),*)
    };
}

#[macro_export]
macro_rules! attributes {
    (style: {$($style:tt)*} $($tail:tt)*) => {
        style!{$($style)*}
    };

    (style: $style:ident $($tail:tt)*) => {
        $style
    };
}

#[macro_export]
macro_rules! element {
    () => {EmptyDescriptor};

    // Reverses child list, so that traversal still happens top down rather than bottom up
    (($head:tt) $($tail:tt)*) => {
        {
            let element = element! {$($tail)*};
            let element = element.add_sibling(element! {$head});
            element
        }
    };
    
    (($tag:ident {$($attrs:tt)*}) $($rest:tt)*) => {
        element! {$tag {$($attrs)*} [] $($rest)*}
    };

    (($tag:ident [$($children:tt)*]) $($rest:tt)*) => {
        element! {$tag {} [$($children)*] $($rest)*}
    };

    (($tag:ident {$($attrs:tt)*} [$($children:tt)*]) $($rest:tt)*) => {
        element! {$tag {$($attrs)*} [$($children)*] $($rest)*}
    };

    (($tag:ident) $($rest:tt)*) => {
        element! {$tag {} [] $($rest)*}
    };

    ($tag:ident) => {
        (EmptyStyles, EmptyDescriptor)
    };

    ($tag:ident {$($attrs:tt)*} [$($children:tt)*] $($rest:tt)*) => {
        {
            let children = element! {$($children)*};
            let element = element! {$($rest)*};
            let element = element.add_sibling::<
                $tag<attribute_type!{$($attrs)*}, element_type! {$($children)*}, element_type! {$($rest)*}>>
                ((attributes! {$($attrs)*}, children));
            element
        }
    };
}

#[macro_export]
macro_rules! attribute_type {
    () => {
        EmptyStyles
    };

    (style: {$($style:tt)*} $($tail:tt)*) => {
        style_type!{$($style)*}
    };

    (style: $style:ident $($tail:tt)*) => {
        StyleNode<EmptyStyles>
    };
}

#[macro_export]
macro_rules! style_type {
    () => {
        EmptyStyles
    };

    ($field:ident: $value:expr, $($rest:tt)*) => {
        StyleNode<style_type!{$($rest)*}>
    };

    ($field:ident: $value:expr) => {
        StyleNode<EmptyStyles>
    };
}

macro_rules! element_type {
    () => {
        EmptyDescriptor
    };

    (($tag:ident {$($attrs:tt)*}) $($rest:tt)*) => {
        element_type! {$tag {$($attrs)*} [] $($rest)*}
    };

    (($tag:ident [$($children:tt)*]) $($rest:tt)*) => {
        element_type! {$tag {} [$($children)*] $($rest)*}
    };

    (($tag:ident {$($attrs:tt)*} [$($children:tt)*]) $($rest:tt)*) => {
        element_type! {$tag {$($attrs)*} [$($children)*] $($rest)*}
    };

    (($tag:ident) $($rest:tt)*) => {
        element_type! {$tag {} [] $($rest)*}
    };

    ($tag:ident) => {
        ComponentDescriptor<$tag<EmptyStyles, EmptyDescriptor, EmptyDescriptor>, EmptyStyles, EmptyDescriptor, EmptyDescriptor>
    };

    ($tag:ident {$($attrs:tt)*} [$($children:tt)*] $($rest:tt)*) => {
        ComponentDescriptor<$tag<attribute_type!{$($attrs)*}, element_type! {$($children)*}, element_type! {$($rest)*}>>
    };
}

macro_rules! component_type {
    () => {
        EmptyComponent
    };

    (($tag:ident {$($attrs:tt)*}) $($rest:tt)*) => {
        component_type! {$tag {$($attrs)*} [] $($rest)*}
    };

    (($tag:ident [$($children:tt)*]) $($rest:tt)*) => {
        component_type! {$tag {} [$($children)*] $($rest)*}
    };

    (($tag:ident {$($attrs:tt)*} [$($children:tt)*]) $($rest:tt)*) => {
        component_type! {$tag {$($attrs)*} [$($children)*] $($rest)*}
    };

    (($tag:ident) $($rest:tt)*) => {
        component_type! {$tag {} [] $($rest)*}
    };

    ($tag:ident) => {
        <$tag<EmptyStyles, EmptyDescriptor, EmptyDescriptor> as ComponentConstructor>::Component
    };

    ($tag:ident {$($attrs:tt)*} [$($children:tt)*] $($rest:tt)*) => {
        <$tag<attribute_type!{$($attrs)*}, element_type! {$($children)*}, element_type! {$($rest)*}> as ComponentConstructor>::Component
    };
}

#[macro_export]
macro_rules! component {
    ($name: ident, |props| {$($rest:tt)*}) => {
        type ElementType = element_type! {$($rest)*};
        type ComponentType = component_type! {$($rest)*};
        pub struct $name {
            element: ElementType,
            component: ComponentType,
            parent_id: i32
        }

        impl $name {
            fn new(parent_id: i32) -> $name {
                let descriptor = element! {$($rest)*};
                let mut component = descriptor.construct_component();
                component.mount(parent_id, &descriptor);
                $name {
                    element: descriptor,
                    component: component,
                    parent_id: parent_id
                }
            }

            fn update(&mut self) {
                let next_descriptor = element! {$($rest)*};
                self.component.update(self.parent_id, &self.element, &next_descriptor);
                self.element = next_descriptor;
            }
        }
    };
}

component! {TestApp,
    |props| {
        (div {style: {width: "500px", height: "500px", backgroundColor: "#FF0000"}} [
            (div {style: {width: "10px", height: "10px", backgroundColor: "#0000FF"}})
            (div)
            (div {style: {width: "100px", height: "100px", backgroundColor: "#000000"}} [
                (div {style: {width: "50px", height: "50px", backgroundColor: "#00FF00"}})
                (div {style: {width: "50px", height: "50px", backgroundColor: "#00FF00"}})
            ])
        ])
    }
}

// Component
use std::marker::PhantomData;
trait Component {
    type Descriptor;
    fn mount(&mut self, parent_id: i32, descriptor: &Self::Descriptor);
    fn update(&mut self, parent_id: i32, prev_descriptor: &Self::Descriptor, descriptor: &Self::Descriptor);
    fn unmount(&mut self);
}

struct EmptyComponent;
impl Component for EmptyComponent {
    type Descriptor = EmptyDescriptor;
    fn mount(&mut self, parent_id: i32, descriptor: &Self::Descriptor) {}
    fn update(&mut self, parent_id: i32, prev_descriptor: &Self::Descriptor, descriptor: &Self::Descriptor) {}
    fn unmount(&mut self) {}
}
// Component

// Constructor
trait ComponentConstructor {
    type Component;
    type Descriptor;
    type Arguments;
    type ChildDescriptor;
    type SiblingDescriptor;

    fn construct_component(descriptor: &Self::Descriptor) -> Self::Component;
}

impl ComponentConstructor for () {
    type Component = EmptyComponent;
    type Descriptor = EmptyDescriptor;
    type Arguments = ();
    type ChildDescriptor = EmptyDescriptor;
    type SiblingDescriptor = EmptyDescriptor;
    fn construct_component(descriptor: &Self::Descriptor) -> Self::Component {
        EmptyComponent
    }
}
// Constructor

// Descriptor
trait Descriptor {
    type Constructor: ComponentConstructor;
    type Arguments;
    type ChildDescriptor: Descriptor;
    type SiblingDescriptor: Descriptor;
    fn construct_component(&self) -> <<Self as Descriptor>::Constructor as ComponentConstructor>::Component;
    fn visit<VisitorFunc>(&self, func: VisitorFunc) where VisitorFunc: FnOnce(&Self::Arguments, &Self::ChildDescriptor, &Self::SiblingDescriptor);
}

struct ComponentDescriptor<Constructor> where Constructor: ComponentConstructor {
    arguments: Constructor::Arguments,
    children: Constructor::ChildDescriptor,
    siblings: Constructor::SiblingDescriptor,
    constructor: PhantomData<Constructor>
}

impl<Constructor> ComponentDescriptor<Constructor> where Constructor: ComponentConstructor {
    #[inline]
    pub fn add_sibling<SiblingConstructor>(self, args: (SiblingConstructor::Arguments, SiblingConstructor::ChildDescriptor))
        -> ComponentDescriptor<SiblingConstructor>
        where SiblingConstructor: ComponentConstructor<SiblingDescriptor=Self>,
              SiblingConstructor::Arguments: Styles,
              SiblingConstructor::ChildDescriptor: Descriptor
    {
        let (arguments, children) = args;
        ComponentDescriptor {
            arguments: arguments,
            children: children,
            siblings: self,
            constructor: PhantomData
        }
    }
}

struct EmptyDescriptor;
impl EmptyDescriptor {
    #[inline]
    pub fn add_sibling<SiblingConstructor>(self, args: (SiblingConstructor::Arguments, SiblingConstructor::ChildDescriptor))
        -> ComponentDescriptor<SiblingConstructor>
        where SiblingConstructor: ComponentConstructor<SiblingDescriptor=Self>,
              SiblingConstructor::Arguments: Styles,
              SiblingConstructor::ChildDescriptor: Descriptor
    {
        let (arguments, children) = args;
        ComponentDescriptor {
            arguments: arguments,
            children: children,
            siblings: self,
            constructor: PhantomData
        }
    }
}

impl Descriptor for EmptyDescriptor {
    type Constructor = ();
    type Arguments = ();
    type ChildDescriptor = EmptyDescriptor;
    type SiblingDescriptor = EmptyDescriptor;
    fn construct_component(&self) -> <<Self as Descriptor>::Constructor as ComponentConstructor>::Component {
        EmptyComponent
    }

    fn visit<VisitorFunc>(&self, func: VisitorFunc) where VisitorFunc: FnOnce(&Self::Arguments, &Self::ChildDescriptor, &Self::SiblingDescriptor) {

    }
}

impl<Constructor> Descriptor for ComponentDescriptor<Constructor>
    where Constructor: ComponentConstructor<Descriptor=ComponentDescriptor<Constructor>>,
          Constructor::Arguments: Styles,
          Constructor::ChildDescriptor: Descriptor,
          Constructor::SiblingDescriptor: Descriptor {
    type Constructor = Constructor;
    type Arguments = Constructor::Arguments;
    type ChildDescriptor = Constructor::ChildDescriptor;
    type SiblingDescriptor = Constructor::SiblingDescriptor;

    fn construct_component(&self) -> <<Self as Descriptor>::Constructor as ComponentConstructor>::Component {
        Constructor::construct_component(self)
    }

    fn visit<VisitorFunc>(&self, func: VisitorFunc) where VisitorFunc: FnOnce(&Self::Arguments, &Self::ChildDescriptor, &Self::SiblingDescriptor) {
        func(&self.arguments, &self.children, &self.siblings);
    }
}
// Descriptor

// DOM
trait DOMDescriptor: Descriptor {
    fn tag() -> HTMLTag;
}

struct DOMComponent<Descriptor, ChildComponent, SiblingComponent> {
    descriptor: PhantomData<Descriptor>,
    dom_handle: i32,
    children: ChildComponent,
    siblings: SiblingComponent
}

struct div<Arguments, ChildDescriptor, SiblingDescriptor> {
    arguments: PhantomData<Arguments>,
    children: PhantomData<ChildDescriptor>,
    siblings: PhantomData<SiblingDescriptor>
}

impl<Arguments, ChildDescriptor, SiblingDescriptor> ComponentConstructor for div<Arguments, ChildDescriptor, SiblingDescriptor> where ChildDescriptor: Descriptor, SiblingDescriptor: Descriptor {
    type Descriptor = ComponentDescriptor<Self>;
    type Component = DOMComponent<Self::Descriptor, <<ChildDescriptor as Descriptor>::Constructor as ComponentConstructor>::Component, <<SiblingDescriptor as Descriptor>::Constructor as ComponentConstructor>::Component>;
    type Arguments = Arguments;
    type ChildDescriptor = ChildDescriptor;
    type SiblingDescriptor = SiblingDescriptor;

    fn construct_component(descriptor: &Self::Descriptor) -> Self::Component {
        DOMComponent {
            descriptor: PhantomData,
            dom_handle: -1,
            children: descriptor.children.construct_component(),
            siblings: descriptor.siblings.construct_component()
        }
    }
}

impl<Arguments, ChildDescriptor, SiblingDescriptor> DOMDescriptor for ComponentDescriptor<div<Arguments, ChildDescriptor, SiblingDescriptor>> 
    where Arguments: Styles,
          ChildDescriptor: Descriptor,
          SiblingDescriptor: Descriptor {
    fn tag() -> HTMLTag {
        HTMLTag::div
    }
}

impl<DescriptorT, ChildComponent, SiblingComponent> Component for DOMComponent<DescriptorT, ChildComponent, SiblingComponent>
    where DescriptorT: DOMDescriptor<ChildDescriptor=ChildComponent::Descriptor, SiblingDescriptor=SiblingComponent::Descriptor>,
          DescriptorT::ChildDescriptor: Descriptor,
          DescriptorT::SiblingDescriptor: Descriptor,
          DescriptorT::Arguments: Styles,
          ChildComponent: Component,
          SiblingComponent: Component {
    type Descriptor = DescriptorT;

    fn mount(&mut self, parent_id: i32, descriptor: &Self::Descriptor) {
        self.dom_handle = unsafe {
            let dom_handle = create_element(Self::Descriptor::tag());
            set_element_parent(dom_handle, parent_id);
            dom_handle
        };

        descriptor.visit(|arguments, children, siblings| {
            arguments.apply(self.dom_handle);
            self.children.mount(self.dom_handle, children);
            self.siblings.mount(parent_id, siblings);
        });
    }

    fn update(&mut self, parent_id: i32, prev_descriptor: &Self::Descriptor, descriptor: &Self::Descriptor) {
        prev_descriptor.visit(|prev_arguments, prev_children, prev_siblings| {
            descriptor.visit(|arguments, children, siblings| {
                arguments.diff_apply(prev_arguments, self.dom_handle);
                self.children.update(self.dom_handle, prev_children, children);
                self.siblings.update(parent_id, prev_siblings, siblings);
            });
        });
    }

    fn unmount(&mut self) {
        self.children.unmount();
        self.siblings.unmount();
    }
}
// DOM

use std::ptr;
use std::default::Default;
#[no_mangle]
pub fn run_demo(prev: *mut TestApp) -> *mut TestApp {
    let mount_start = unsafe{now()};
    let body = unsafe {get_document_body()};

    let mut app = if !prev.is_null() {
        unsafe{Box::from_raw(prev)}
    } else {
        Box::new(TestApp::new(body))
    };
    (*app).update();
    let mount_end = unsafe{now()};

    println!("Elapsed: {}", (mount_end - mount_start));
    Box::into_raw(app)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
