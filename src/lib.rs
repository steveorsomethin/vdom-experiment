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
use std::mem::forget;

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
    fn reset_element_style(element_id: i32, style: CSSStyle) -> i32;
    fn set_element_style_f32(element_id: i32, style: CSSStyle, value: f32) -> i32;
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
    fn diff_apply<'a, P>(&'a mut self, prev_styles: &P, element_id: i32) where P: Styles;
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F);
    fn compare(&self, name: CSSStyle, value: &str) -> bool;
}

#[derive(Debug)]
struct EmptyStyles;
impl Styles for EmptyStyles {
    type R = EmptyStyles;

    #[inline]
    fn apply<'a>(&'a self, _: i32) {
    }

    #[inline]
    fn reset<'a>(&'a self, element_id: i32) {

    }

    #[inline]
    fn diff_apply<'a, P>(&'a mut self, prev_styles: &P, element_id: i32) where P: Styles {
        prev_styles.reset(element_id);
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&EmptyStyles);
    }

    #[inline]
    fn compare(&self, name: CSSStyle, value: &str) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct StyleNode<'n, R> where R: Styles {
    name: CSSStyle,
    value: &'n str,
    sibling: R
}

impl<'n> StyleNode<'n, EmptyStyles> {
    #[inline]
    pub fn new(name: CSSStyle, value: &'n str,)
               -> StyleNode<'n, EmptyStyles>
    {
        StyleNode {
            name: name,
            value: value,
            sibling: EmptyStyles
        }
    }
}

impl<'n, R> StyleNode<'n, R> where R: Styles {
    #[inline]
    pub fn add_sibling(self, name: CSSStyle, value: &'n str)
                  -> StyleNode<'n, StyleNode<'n, R>>
    {
        StyleNode {
            name: name,
            value: value,
            sibling: self
        }
    }
}

impl<'n, R> Styles for StyleNode<'n, R> where R: Styles {
    type R = R;
    #[inline(always)]
    fn apply<'a>(&'a self, element_id: i32) {
        self.sibling.apply(element_id);
        unsafe {
            set_element_style_str(element_id, self.name, CString::new(self.value).unwrap().as_ptr(), self.value.len() as i32);
        }
    }

    #[inline]
    fn reset<'a>(&'a self, element_id: i32) {
        unsafe {
            reset_element_style(element_id, self.name);
        }
    }

    #[inline(always)]
    fn diff_apply<'a, P>(&'a mut self, prev_styles: &P, element_id: i32) where P: Styles {
        prev_styles.apply_sibling(|sibling| self.sibling.diff_apply(sibling, element_id));

        if !prev_styles.compare(self.name, self.value) {
            unsafe {
                set_element_style_str(element_id, self.name, CString::new(self.value).unwrap().as_ptr(), self.value.len() as i32);
            }
        }
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&self.sibling);
    }

    #[inline]
    fn compare(&self, name: CSSStyle, value: &str) -> bool {
        self.name == name && self.value == value
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

// Children
pub trait Elements where Self::U: Styles, Self::C: Elements, Self::R: Elements {
    type U;
    type C;
    type R;
    fn apply<'a>(&'a mut self, parent_id: i32);
    fn diff_apply<'a, P>(&'a mut self, prev_element: &P, parent_id: i32) where P: Elements, P::C: Elements;
    fn unmount(&self);
    fn apply_children<'a, F: FnOnce(&Self::C)>(&self, f: F);
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F);
    fn apply_styles<'a, F: FnOnce(&Self::U)>(&self, f: F);
    fn tag(&self) -> HTMLTag;
    fn element_id(&self) -> i32;
}

#[derive(Debug)]
struct EmptyElements;
impl EmptyElements {
    #[inline]
    pub fn add_sibling<G, U>(self, args: (HTMLTag, G, U))
                  -> ElementNode<U, G, EmptyElements>
                  where G: Elements, U: Styles
    {
        let (tag, children, styles) = args;
        ElementNode {
            tag: tag,
            element_id: -1,
            styles: styles,
            children: children,
            sibling: self
        }
    }
}

impl Elements for EmptyElements {
    type U = EmptyStyles;
    type C = EmptyElements;
    type R = EmptyElements;

    #[inline(always)]
    fn apply<'a>(&'a mut self, _: i32) {

    }

    #[inline]
    fn unmount(&self) {

    }

    #[inline(always)]
    fn diff_apply<'a, P>(&'a mut self, prev_element: &P, parent_id: i32) where P: Elements, P::C: Elements {
        let element_id = prev_element.element_id();
        if prev_element.tag() != self.tag() {
            unsafe {
                destroy_element(element_id);
            }
        }
    }

    #[inline]
    fn apply_children<'a, F: FnOnce(&Self::C)>(&self, f: F) {
        f(&EmptyElements);
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&EmptyElements);
    }

    #[inline]
    fn apply_styles<'a, F: FnOnce(&Self::U)>(&self, f: F) {
        f(&EmptyStyles);
    }

    #[inline]
    fn tag(&self) -> HTMLTag {
        HTMLTag::EMPTY
    }

    #[inline]
    fn element_id(&self) -> i32 {
        -1
    }
}

#[derive(Debug)]
pub struct ElementNode<S, C, R> where S: Styles, C: Elements, R: Elements {
    tag: HTMLTag,
    element_id: i32,
    styles: S,
    children: C,
    sibling: R
}

impl<S, C> ElementNode<S, C, EmptyElements> where S: Styles, C: Elements {
    #[inline]
    pub fn new(tag: HTMLTag, children: C, styles: S)
               -> ElementNode<S, C, EmptyElements>
    {
        ElementNode {
            tag: tag,
            element_id: -1,
            styles: styles,
            children: children,
            sibling: EmptyElements
        }
    }
}

impl<S, C, R> ElementNode<S, C, R> where S: Styles, C: Elements, R: Elements {
    #[inline]
    pub fn add_sibling<G, U>(self, args: (HTMLTag, G, U))
                  -> ElementNode<U, G, ElementNode<S, C, R>>
                  where G: Elements, U: Styles
    {
        let (tag, children, styles) = args;
        ElementNode {
            tag: tag,
            element_id: -1,
            styles: styles,
            children: children,
            sibling: self
        }
    }
}

impl<S, C, R> Elements for ElementNode<S, C, R> where S: Styles, C: Elements, R: Elements {
    type U = S;
    type C = C;
    type R = R;

    #[inline(always)]
    fn apply<'a>(&'a mut self, parent_id: i32) {
        let element_id = unsafe {
            let element_id = create_element(self.tag);
            set_element_parent(element_id, parent_id);
            self.styles.apply(element_id);
            element_id
        };

        self.children.apply(element_id);
        self.sibling.apply(parent_id);
        self.element_id = element_id;
    }

    #[inline(always)]
    fn diff_apply<'a, P>(&'a mut self, prev_element: &P, parent_id: i32) where P: Elements {
        let element_id = prev_element.element_id();
        if prev_element.tag() == self.tag() {
            self.element_id = element_id;
            prev_element.apply_styles(|prev_styles| {
                self.styles.diff_apply(prev_styles, element_id);
            });

            prev_element.apply_children(|prev_children| {
                self.children.diff_apply(prev_children, element_id);
            });
            
            prev_element.apply_sibling(|prev_sibling| {
                self.sibling.diff_apply(prev_sibling, parent_id);
            });
        } else {
            prev_element.unmount();
            self.apply(parent_id);
        }
    }

    #[inline]
    fn unmount(&self) {
        unsafe {
            destroy_element(self.element_id);
        }
    }

    #[inline]
    fn apply_children<'a, F: FnOnce(&Self::C)>(&self, f: F) {
        f(&self.children);
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        f(&self.sibling);
    }

    #[inline]
    fn apply_styles<'a, F: FnOnce(&Self::U)>(&self, f: F) {
        f(&self.styles);
    }

    #[inline]
    fn tag(&self) -> HTMLTag {
        self.tag
    }

    #[inline]
    fn element_id(&self) -> i32 {
        self.element_id
    }
}

use std::ops::{Deref, DerefMut};
impl<I, D> Elements for D where D: DerefMut<Target=I>, D: Deref<Target=I>, I: Elements {
    type U = I::U;
    type C = I::C;
    type R = I::R;

    #[inline]
    fn apply<'a>(&'a mut self, element_id: i32) {
        (**self).apply(element_id);
    }

    #[inline]
    fn unmount(&self) {
        (**self).unmount();
    }

    #[inline]
    fn diff_apply<'a, P>(&'a mut self, prev_element: &P, parent_id: i32) where P: Elements, P::C: Elements {
        (**self).diff_apply(prev_element, parent_id);
    }

    #[inline]
    fn apply_children<'a, F: FnOnce(&Self::C)>(&self, f: F) {
        (**self).apply_children(f);
    }

    #[inline]
    fn apply_sibling<'a, F: FnOnce(&Self::R)>(&self, f: F) {
        (**self).apply_sibling(f);
    }

    #[inline]
    fn apply_styles<'a, F: FnOnce(&Self::U)>(&self, f: F) {
        (**self).apply_styles(f);
    }

    #[inline]
    fn tag(&self) -> HTMLTag {
        (**self).tag()
    }

    #[inline]
    fn element_id(&self) -> i32 {
        (**self).element_id()
    }
}
// Children

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
    () => {EmptyElements};

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
        (HTMLTag::$tag, EmptyElements, EmptyStyles)
    };

    ($tag:ident {$($attrs:tt)*} [$($children:tt)*] $($rest:tt)*) => {
        {
            let children = element! {$($children)*};
            let element = element! {$($rest)*};
            let element = element.add_sibling((HTMLTag::$tag, children, attributes! {$($attrs)*}));
            element
        }
    };
}

#[no_mangle]
pub fn run_demo(diff: bool) {
    let mount_start = unsafe{now()};
    let body = unsafe {get_document_body()};
    let mut element =  element! {
        (div {style: {width: "500px", height: "500px", backgroundColor: "yellow"}} [
            (div {style: {width: "10px", height: "10px", backgroundColor: "blue"}})
            (div)
            (div {style: {width: "100px", height: "100px", backgroundColor: "black"}} [
                (div {style: {width: "50px", height: "50px", backgroundColor: "green"}})
            ])
        ])
    };
    element.diff_apply(&EmptyElements, body);
    let mount_end = unsafe{now()};

    if !diff {
        println!("Mount Elapsed: {}", (mount_end - mount_start));
        return;
    }

    let diff_start = unsafe{now()};
    let prev_element = element;
    let mut element =  element! {
        (div {style: {width: "500px", height: "500px", backgroundColor: "yellow"}} [
            (div {style: {width: "10px", height: "10px", backgroundColor: "blue"}})
            (div)
            (div {style: {width: "100px", height: "100px", backgroundColor: "black"}} [
                (div {style: {width: "50px", height: "50px", backgroundColor: "green"}})
            ])
        ])
    };
    element.diff_apply(&prev_element, body);
    let diff_end = unsafe{now()};
    println!("Mount Elapsed: {}", (mount_end - mount_start));
    println!("Diff Elapsed: {}", (diff_end - diff_start));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
