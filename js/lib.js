var elementTypes = [
    'a',
    'abbr',
    'address',
    'area',
    'article',
    'aside',
    'audio',
    'b',
    'base',
    'bdi',
    'bdo',
    'blockquote',
    'body',
    'br',
    'button',
    'canvas',
    'caption',
    'cite',
    'code',
    'col',
    'colgroup',
    'data',
    'datalist',
    'dd',
    'del',
    'details',
    'dfn',
    'div',
    'dl',
    'dt',
    'element',
    'em',
    'embed',
    'fieldset',
    'figcaption',
    'figure',
    'footer',
    'form',
    'head',
    'header',
    'hr',
    'html',
    'i',
    'iframe',
    'img',
    'input',
    'ins',
    'kbd',
    'label',
    'legend',
    'li',
    'link',
    'main',
    'map',
    'mark',
    'meta',
    'meter',
    'nav',
    'noframes',
    'noscript',
    'object',
    'ol',
    'optgroup',
    'option',
    'output',
    'p',
    'param',
    'pre',
    'progress',
    'q',
    'rp',
    'rt',
    'rtc',
    'ruby',
    's',
    'samp',
    'script',
    'section',
    'select',
    'shadow',
    'small',
    'source',
    'span',
    'strong',
    'style',
    'sub',
    'summary',
    'sup',
    'table',
    'tbody',
    'td',
    'template',
    'textarea',
    'tfoot',
    'th',
    'thead',
    'time',
    'title',
    'tr',
    'track',
    'u',
    'ul',
    'var',
    'video',
    'wbr'
];

var attributeTypes = [
    'accept',
    'acceptCharset',
    'accesskey',
    'action',
    'align',
    'alt',
    'async',
    'autocomplete',
    'autofocus',
    'autoplay',
    'autosave',
    'bgcolor',
    'border',
    'buffered',
    'challenge',
    'charset',
    'checked',
    'cite',
    'class',
    'code',
    'codebase',
    'color',
    'cols',
    'colspan',
    'content',
    'contenteditable',
    'contextmenu',
    'controls',
    'coords',
    'data',
    'dataDash',
    'datetime',
    'default',
    'defer',
    'dir',
    'dirname',
    'disabled',
    'download',
    'draggable',
    'dropzone',
    'enctype',
    'for',
    'form',
    'formaction',
    'headers',
    'height',
    'hidden',
    'high',
    'href',
    'hreflang',
    'httpEquiv',
    'icon',
    'id',
    'ismap',
    'itemprop',
    'keytype',
    'kind',
    'label',
    'lang',
    'language',
    'list',
    'loop',
    'low',
    'manifest',
    'max',
    'maxlength',
    'media',
    'method',
    'min',
    'multiple',
    'muted',
    'name',
    'novalidate',
    'open',
    'optimum',
    'pattern',
    'ping',
    'placeholder',
    'poster',
    'preload',
    'radiogroup',
    'readonly',
    'rel',
    'required',
    'reversed',
    'rows',
    'rowspan',
    'sandbox',
    'scope',
    'scoped',
    'seamless',
    'selected',
    'shape',
    'size',
    'sizes',
    'span',
    'spellcheck',
    'src',
    'srcdoc',
    'srclang',
    'srcset',
    'start',
    'step',
    'style',
    'summary',
    'tabindex',
    'target',
    'title',
    'type',
    'usemap',
    'value',
    'width',
    'wrap'
];

var attributeSetters = attributeTypes.map(function (key) {
    return new Function('element', 'value', 'element.' + key + ' = value;');
});

var styleTypes = [
    'alignContent',
    'alignItems',
    'alignSelf',
    'alignmentBaseline',
    'all',
    'animation',
    'animationDelay',
    'animationDirection',
    'animationDuration',
    'animationFillMode',
    'animationIterationCount',
    'animationName',
    'animationPlayState',
    'animationTimingFunction',
    'backfaceVisibility',
    'background',
    'backgroundAttachment',
    'backgroundBlendMode',
    'backgroundClip',
    'backgroundColor',
    'backgroundImage',
    'backgroundOrigin',
    'backgroundPosition',
    'backgroundPositionX',
    'backgroundPositionY',
    'backgroundRepeat',
    'backgroundRepeatX',
    'backgroundRepeatY',
    'backgroundSize',
    'baselineShift',
    'border',
    'borderBottom',
    'borderBottomColor',
    'borderBottomLeftRadius',
    'borderBottomRightRadius',
    'borderBottomStyle',
    'borderBottomWidth',
    'borderCollapse',
    'borderColor',
    'borderImage',
    'borderImageOutset',
    'borderImageRepeat',
    'borderImageSlice',
    'borderImageSource',
    'borderImageWidth',
    'borderLeft',
    'borderLeftColor',
    'borderLeftStyle',
    'borderLeftWidth',
    'borderRadius',
    'borderRight',
    'borderRightColor',
    'borderRightStyle',
    'borderRightWidth',
    'borderSpacing',
    'borderStyle',
    'borderTop',
    'borderTopColor',
    'borderTopLeftRadius',
    'borderTopRightRadius',
    'borderTopStyle',
    'borderTopWidth',
    'borderWidth',
    'bottom',
    'boxShadow',
    'boxSizing',
    'breakAfter',
    'breakBefore',
    'breakInside',
    'bufferedRendering',
    'captionSide',
    'clear',
    'clip',
    'clipPath',
    'clipRule',
    'color',
    'colorInterpolation',
    'colorInterpolationFilters',
    'colorRendering',
    'columnCount',
    'columnFill',
    'columnGap',
    'columnRule',
    'columnRuleColor',
    'columnRuleStyle',
    'columnRuleWidth',
    'columnSpan',
    'columnWidth',
    'columns',
    'contain',
    'content',
    'counterIncrement',
    'counterReset',
    'cursor',
    'cx',
    'cy',
    'd',
    'direction',
    'display',
    'dominantBaseline',
    'emptyCells',
    'fill',
    'fillOpacity',
    'fillRule',
    'filter',
    'flex',
    'flexBasis',
    'flexDirection',
    'flexFlow',
    'flexGrow',
    'flexShrink',
    'flexWrap',
    'float',
    'floodColor',
    'floodOpacity',
    'font',
    'fontFamily',
    'fontFeatureSettings',
    'fontKerning',
    'fontSize',
    'fontStretch',
    'fontStyle',
    'fontVariant',
    'fontVariantCaps',
    'fontVariantLigatures',
    'fontVariantNumeric',
    'fontWeight',
    'height',
    'imageRendering',
    'isolation',
    'justifyContent',
    'left',
    'letterSpacing',
    'lightingColor',
    'lineHeight',
    'listStyle',
    'listStyleImage',
    'listStylePosition',
    'listStyleType',
    'margin',
    'marginBottom',
    'marginLeft',
    'marginRight',
    'marginTop',
    'marker',
    'markerEnd',
    'markerMid',
    'markerStart',
    'mask',
    'maskType',
    'maxHeight',
    'maxWidth',
    'maxZoom',
    'minHeight',
    'minWidth',
    'minZoom',
    'mixBlendMode',
    'motion',
    'motionOffset',
    'motionPath',
    'motionRotation',
    'objectFit',
    'objectPosition',
    'opacity',
    'order',
    'orientation',
    'orphans',
    'outline',
    'outlineColor',
    'outlineOffset',
    'outlineStyle',
    'outlineWidth',
    'overflow',
    'overflowWrap',
    'overflowX',
    'overflowY',
    'padding',
    'paddingBottom',
    'paddingLeft',
    'paddingRight',
    'paddingTop',
    'page',
    'pageBreakAfter',
    'pageBreakBefore',
    'pageBreakInside',
    'paintOrder',
    'perspective',
    'perspectiveOrigin',
    'pointerEvents',
    'position',
    'quotes',
    'r',
    'resize',
    'right',
    'rx',
    'ry',
    'shapeImageThreshold',
    'shapeMargin',
    'shapeOutside',
    'shapeRendering',
    'size',
    'speak',
    'src',
    'stopColor',
    'stopOpacity',
    'stroke',
    'strokeDasharray',
    'strokeDashoffset',
    'strokeLinecap',
    'strokeLinejoin',
    'strokeMiterlimit',
    'strokeOpacity',
    'strokeWidth',
    'tabSize',
    'tableLayout',
    'textAlign',
    'textAlignLast',
    'textAnchor',
    'textCombineUpright',
    'textDecoration',
    'textIndent',
    'textOrientation',
    'textOverflow',
    'textRendering',
    'textShadow',
    'textSizeAdjust',
    'textTransform',
    'top',
    'touchAction',
    'transform',
    'transformOrigin',
    'transformStyle',
    'transition',
    'transitionDelay',
    'transitionDuration',
    'transitionProperty',
    'transitionTimingFunction',
    'unicodeBidi',
    'unicodeRange',
    'userZoom',
    'vectorEffect',
    'verticalAlign',
    'visibility',
    'whiteSpace',
    'widows',
    'width',
    'willChange',
    'wordBreak',
    'wordSpacing',
    'wordWrap',
    'writingMode',
    'x',
    'y',
    'zIndex',
    'zoom'
];

var styleSetters = styleTypes.map(function (key) {
    return new Function('style', 'value', 'style.' + key + ' = value;');
});

var eventTypes = [
    'onabort',
    'onblur',
    'oncancel',
    'oncanplay',
    'oncanplaythrough',
    'onchange',
    'onclick',
    'onclose',
    'oncontextmenu',
    'oncuechange',
    'ondblclick',
    'ondrag',
    'ondragend',
    'ondragenter',
    'ondragleave',
    'ondragover',
    'ondragstart',
    'ondrop',
    'ondurationchange',
    'onemptied',
    'onended',
    'onerror',
    'onfocus',
    'oninput',
    'oninvalid',
    'onkeydown',
    'onkeypress',
    'onkeyup',
    'onload',
    'onloadeddata',
    'onloadedmetadata',
    'onloadstart',
    'onmousedown',
    'onmouseenter',
    'onmouseleave',
    'onmousemove',
    'onmouseout',
    'onmouseover',
    'onmouseup',
    'onmousewheel',
    'onpause',
    'onplay',
    'onplaying',
    'onprogress',
    'onratechange',
    'onreset',
    'onresize',
    'onscroll',
    'onseeked',
    'onseeking',
    'onselect',
    'onshow',
    'onstalled',
    'onsubmit',
    'onsuspend',
    'ontimeupdate',
    'ontoggle',
    'onvolumechange',
    'onwaiting',
    'onbeforecopy',
    'onbeforecut',
    'onbeforepaste',
    'oncopy',
    'oncut',
    'onpaste',
    'onsearch',
    'onselectstart',
    'onwheel'
];

var eventSetters = eventTypes.map(function (key) {
    return new Function('element', 'value', 'element.' + key + ' = value;');
});

var externs = {
    $documentId: undefined,
    $documentBodyId: undefined,
    $elementHandles: {},
    $nextElementID: 0,
    $elementTypes: elementTypes,
    $styleTypes: styleTypes,
    $styleSetters: styleSetters,
    $attributeTypes: attributeTypes,
    $attributeSetters: attributeSetters,
    $eventTypes: eventTypes,
    $eventSetters: eventSetters,
    $OK: 0,
    $INVALID_ID: 1,

    get_document: function getDocument() {
        //false && console.warn('getDocument');
        if (documentId !== undefined) {
            return documentId;
        }
        documentId = nextElementID++;
        documentBodyId = nextElementID++;
        elementHandles[documentId] = document;
        elementHandles[documentBodyId] = document.body;
        return documentId;
    },
    get_document_body: function getDocumentBody() {
        //false && console.warn('getDocumentBody');
        if (documentBodyId !== undefined) {
            return documentBodyId;
        }
        documentId = nextElementID++;
        documentBodyId = nextElementID++;
        elementHandles[documentId] = document;
        elementHandles[documentBodyId] = document.body;
        return documentBodyId;
    },
    create_element: function createElement(type) {
        //false && console.warn('createElement');
        var tagName = elementTypes[type];
        if (tagName === undefined) {
            return -1;
        }

        var elementId = nextElementID++;
        elementHandles[elementId] = document.createElement(tagName);
        return elementId;
    },
    destroy_element: function destroyElement(elementId) {
        //false && console.warn('destroyElement');
        var element = elementHandles[elementId];
        element.parentNode.removeChild(element);

        delete elementHandles[elementId];

        return OK;
    },
    set_element_parent: function setElementParent(elementId, parentElementId) {
        //false && console.warn('setElementParent');
        var element = elementHandles[elementId];
        var parentElement = elementHandles[parentElementId];

        parentElement.appendChild(element);

        return OK;
    },
    set_element_attribute_str: function setElementAttributeStr(elementId, attributeType, attributeValue, strLen) {
        //false && console.warn('setElementAttributeStr');
        var element = elementHandles[elementId];
        var setter = attributeSetters[attributeType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element, Pointer_stringify(styleValue, strLen));

        return OK;
    },
    set_element_attribute_f32: function setElementAttributeFloat(elementId, attributeType, attributeValue) {
        //false && console.warn('setElementAttributeFloat');
        var element = elementHandles[elementId];
        var setter = attributeSetters[attributeType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element, attributeValue);

        return OK;
    },
    set_element_style_str: function setElementStyleStr(elementId, styleType, styleValue, strLen) {
        //false && console.warn('setElementStyleStr');
        var element = elementHandles[elementId];
        var setter = styleSetters[styleType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element.style, Pointer_stringify(styleValue, strLen));

        return OK;
    },
    reset_element_style: function resetElementStyle(elementId, styleType) {
        //false && console.warn('resetElementStyle');
        var element = elementHandles[elementId];
        var setter = styleSetters[styleType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element.style, '');

        return OK;
    },
    set_element_style_f32: function setElementStyleFloat(elementId, styleType, styleValue) {
        //false && console.warn('setElementStyleFloat');
        var element = elementHandles[elementId];
        var setter = styleSetters[styleType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element.style, styleValue);

        return OK;
    },
    add_event_listener: function addEventListener(elementId, eventType, functionPointer, contextPointer) {
        var element = elementHandles[elementId];
        var setter = eventSetters[eventType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        function handler() {
            Runtime.dynCall('vii', functionPointer, [contextPointer, 10]);
        };

        setter(element, handler);

        return OK;
    },
    remove_event_listener: function removeEventListener(elementId, eventType) {
        var element = elementHandles[elementId];
        var setter = eventSetters[eventType];

        if (setter === undefined) {
            return INVALID_ID;
        }

        setter(element, undefined);

        return OK;
    },
    now: function now() {
        return performance.now() * 100;
    }
};

autoAddDeps(externs, '$documentId');
autoAddDeps(externs, '$documentBodyId');
autoAddDeps(externs, '$elementHandles');
autoAddDeps(externs, '$nextElementID');
autoAddDeps(externs, '$elementTypes');
autoAddDeps(externs, '$styleTypes');
autoAddDeps(externs, '$styleSetters');
autoAddDeps(externs, '$attributeTypes');
autoAddDeps(externs, '$attributeSetters');
autoAddDeps(externs, '$eventTypes');
autoAddDeps(externs, '$eventSetters');
autoAddDeps(externs, '$OK');
autoAddDeps(externs, '$INVALID_ID');
mergeInto(LibraryManager.library, externs);